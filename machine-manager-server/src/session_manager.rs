// Copyright (C) 2021 Cartesi Pte. Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

//! Session Manager component, responsible for keeping of list
//! of active sessions
use super::session::{Session, SessionRequest};
use crate::server_manager::{LocalServerManager, ServerManager};
use crate::CARTESI_BIN_PATH;
use async_mutex::Mutex;
use async_trait::async_trait;
use grpc_cartesi_machine::{MachineConfig, MachineRuntimeConfig};
use std::collections::HashMap;
use std::sync::Arc;

use crate::session::WAIT_RETRIES_NUMBER;
use crate::session::WAIT_SLEEP_STEP;

/// Error type returned from session manager functions
#[derive(Debug, Default)]
pub struct SessionManagerError {
    message: String,
}

impl SessionManagerError {
    fn new(message: &str) -> Self {
        SessionManagerError {
            message: String::from(message),
        }
    }
}

impl std::fmt::Display for SessionManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Session manager error: {}", &self.message)
    }
}

impl std::error::Error for SessionManagerError {}

/// Interface for session manager
#[async_trait]
pub trait SessionManager: Send + Sync {
    async fn closing_all_sessions(
        &self
        ) -> Result <(), Box<SessionManagerError>>;
    /// Create new session from provided machine configuration
    /// and machine runtime configuration
    async fn create_session_from_config(
        &mut self,
        session_id: &str,
        config: &MachineConfig,
        runtime_config: &MachineRuntimeConfig,
        request: &SessionRequest,
        force: bool,
    ) -> Result<Arc<Mutex<Session>>, Box<dyn std::error::Error>>;
    /// Create new session from stored machine directory
    async fn create_session_from_directory(
        &mut self,
        session_id: &str,
        directory: &str,
        runtime_config: &MachineRuntimeConfig,
        request: &SessionRequest,
        force: bool,
    ) -> Result<Arc<Mutex<Session>>, Box<dyn std::error::Error>>;
    /// Get active session thread safe reference
    async fn get_session(
        &self,
        session_id: &str,
    ) -> Result<Arc<Mutex<Session>>, Box<dyn std::error::Error + Send>>;

    /// Close session and destroy all internal objects
    async fn close_session(&mut self, session_id: &str) -> Result<(), Box<dyn std::error::Error>>;

    fn get_shutting_down_state(&self) -> bool; 
}

/// Session manager that keeps session information data in
/// volatile memory
pub struct RamSessionManager {
    pub session_list: Mutex<HashMap<String, Arc<Mutex<Session>>>>,
    server_manager: Arc<Mutex<dyn ServerManager>>,
    checkin_address: String,
    shutting_down: bool,
}

impl Default for RamSessionManager {
    fn default() -> Self {
        let cartesi_bin_path = match std::env::var(&CARTESI_BIN_PATH) {
            Ok(path) => path,
            Err(_) => panic!("Environment {} not specified", CARTESI_BIN_PATH),
        };
        RamSessionManager {
            session_list: Mutex::new(HashMap::new()),
            server_manager: Arc::new(Mutex::new(LocalServerManager::new(
                &cartesi_bin_path,
                crate::server_manager::HOST,
            ))),
            checkin_address: "".to_string(),
            shutting_down: false,
        }
    }
}

#[allow(dead_code)]
impl RamSessionManager {
    pub fn new(checkin_address: String, server_manager: &Arc<Mutex<dyn ServerManager>>) -> Self {
        RamSessionManager {
            checkin_address,
            server_manager: Arc::clone(server_manager),
            ..Default::default()
        }
    }
}


/// Internal session manager function that handles session initialization:
/// * Instantiate server using server manager
/// * Wait for check in to happen from new server instance with the info about the port
/// * Setup session connection to newly checked in server
/// * Create new machine on remote-cartesi-machine
/// * Perform snapshot and wait for new checkin
/// * Return successfully
async fn initialize_session(
    session_id: &str,
    session_mut: Arc<Mutex<Session>>,
    checkin_address: &str,
    server_manager: Arc<Mutex<dyn ServerManager>>,
) -> Result<Arc<Mutex<Session>>, Box<dyn std::error::Error>> {
    session_mut
        .lock()
        .await
        .setup_session_cartesi_server(checkin_address)
        .await?;

    //Wait for check in
    for i in 1..=WAIT_RETRIES_NUMBER {
        log::debug!(
            "waiting for checking session_id={} retry={}",
            &session_id,
            i
        );
        std::thread::sleep(std::time::Duration::from_millis(WAIT_SLEEP_STEP));
        let (checked, address) = server_manager.lock().await.get_check_in_status(session_id);
        if checked {
            log::debug!(
                "session session_id={} is checked in on address: {}",
                &session_id,
                &address
            );
            break;
        }

        if i == WAIT_RETRIES_NUMBER {
            session_mut.lock().await.close().await?;
            return Err(Box::new(SessionManagerError::new(&format!(
                "Timeout while waiting for session id={} server checkin",
                session_id
            ))));
        }
    }
    let mut session = session_mut.lock().await;
    // Create connection to new server
    session.setup_connection().await?;
    //After finalized check in, create machine on server
    match session.create_machine().await {
        Ok(()) => {
            session.clear_request();
        }
        Err(err) => {
            return Err(Box::new(SessionManagerError::new(&format!(
                "unable to create machine on new remote cartesi machine: {}",
                err.to_string()
            ))));
        }
    };

    //Perform snapshot and wait for new checkin
    log::debug!(
        "performing snapshot on first cycle for session id {}",
        session_id
    );
    session.snapshot().await?;
    log::debug!("snapshot performed for session id {}", session_id);
    drop(session);

    log::debug!("session with session_id={} created", &session_id);
    Ok(session_mut)
}

#[async_trait]
impl SessionManager for RamSessionManager {

    async fn closing_all_sessions(
        &self
        )-> Result <(), Box<SessionManagerError>>{
        let list = self.session_list.lock().await;
        for session_id in list.keys() {

            log::debug!(
                "Acquiring lock for session {}",
                session_id
            );

            match list.get(session_id) {
                Some(session) => {

                    log::debug!(
                        "Lock for session {} acquired",
                        session_id
                    );

                    &session.lock().await.close();
                    
                    let session_server = session.lock().await;
                    let mut server_manager = session_server.get_server_manager().lock().await;
                    let client = session.lock().await;
                    let mut client = client.cartesi_session_client().clone();
                    server_manager.close_server(&mut client);
                    Ok(())
                }
                None => {
                    Err(Box::new(SessionManagerError::new("unknown session id")))
                }
            };

        }
        Ok(())
      }

    fn get_shutting_down_state (&self) -> bool {
        self.shutting_down
    }
    
    async fn create_session_from_config(
        &mut self,
        session_id: &str,
        config: &MachineConfig,
        runtime_config: &MachineRuntimeConfig,
        request: &SessionRequest,
        force: bool,
    ) -> Result<Arc<Mutex<Session>>, Box<dyn std::error::Error>> {
        let mut list = self.session_list.lock().await;
        if let Some(session) = list.get(session_id) {
            if force {
                log::debug!(
                    "closing existing session session_id={} to recreate session",
                    &session_id
                );
                session.lock().await.close().await?;
                list.remove(session_id).ok_or_else(|| {
                    Box::new(SessionManagerError::new(&format!(
                        "error removing session {} from the list",
                        session_id
                    )))
                })?;
            } else {
                return Err(Box::new(SessionManagerError::new(&format!(
                    "session with id='{}' already exists",
                    &session_id
                ))));
            }
        }

        let session_mut = Arc::new(Mutex::new(
            Session::init_from_config(
                &self.server_manager,
                session_id,
                config,
                runtime_config,
                request,
            )
            .await?,
        ));
        list.insert(session_id.to_string(), Arc::clone(&session_mut));
        drop(list);
        log::info!("session with session_id={} created", &session_id);

        // Get temporary result, to be able to async handle removing session id from the list in case of error
        let response: Result<Arc<Mutex<Session>>, SessionManagerError> = match initialize_session(
            session_id,
            session_mut,
            &self.checkin_address,
            Arc::clone(&self.server_manager),
        )
        .await
        {
            Ok(session_mut) => Ok(session_mut),
            Err(err) => Err(SessionManagerError::new(&format!(
                "error initializing session, details: '{}'",
                err.to_string()
            ))),
        };

        return match response {
            Ok(result) => Ok(result),
            Err(err) => {
                self.session_list
                    .lock()
                    .await
                    .remove(session_id)
                    .ok_or_else(|| {
                        Box::new(SessionManagerError::new(&format!(
                            "error removing session id=\"{}\" while cleaning up session list",
                            session_id
                        )))
                    })?;
                Err(Box::new(err))
            }
        };
    }

    async fn create_session_from_directory(
        &mut self,
        session_id: &str,
        directory: &str,
        runtime_config: &MachineRuntimeConfig,
        request: &SessionRequest,
        force: bool,
    ) -> Result<Arc<Mutex<Session>>, Box<dyn std::error::Error>> {
        let mut list = self.session_list.lock().await;
        if force {
            if let Some(session) = list.get(session_id) {
                log::debug!(
                    "closing existing session session_id={} to recreate session",
                    &session_id
                );
                session.lock().await.close().await?;
                list.remove(session_id).ok_or_else(|| {
                    Box::new(SessionManagerError::new(&format!(
                        "error removing session {} from the list",
                        session_id
                    )))
                })?;
            }
        }
        let session_mut = Arc::new(Mutex::new(
            Session::init_from_directory(
                &self.server_manager,
                session_id,
                directory,
                runtime_config,
                request,
            )
            .await?,
        ));
        list.insert(session_id.to_string(), Arc::clone(&session_mut));
        drop(list);
        initialize_session(
            session_id,
            session_mut,
            &self.checkin_address,
            Arc::clone(&self.server_manager),
        )
        .await
    }

    async fn get_session(
        &self,
        session_id: &str,
    ) -> Result<Arc<Mutex<Session>>, Box<dyn std::error::Error + Send>> {
        let list = self.session_list.lock().await;
        match list.get(session_id) {
            Some(session) => Ok(Arc::clone(session)),
            None => Err(Box::new(SessionManagerError::new(&format!(
                "no session with id='{}'",
                session_id
            )))),
        }
    }

    async fn close_session(&mut self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut list = self.session_list.lock().await;
        match list.get(session_id) {
            Some(session) => {
                log::debug!("closing session with id='{}'", session_id);
                session.lock().await.close().await?;
            }
            None => {
                return Err(Box::new(SessionManagerError::new("unknown session id")));
            }
        };
        list.remove(session_id).ok_or_else(|| {
            Box::new(SessionManagerError::new(&format!(
                "error removing session id='{}' from the list",
                session_id
            )))
        })?;
        Ok(())
    }
}
