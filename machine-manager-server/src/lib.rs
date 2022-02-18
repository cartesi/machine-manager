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

//! Implementation of the Machine Mananer service api and
//! Check in service api, defined in machine-manager.proto and
//! cartesi-machine-checkin.proto

pub mod server_manager;
pub mod session;
pub mod session_manager;

use crate::server_manager::ServerManager;
use crate::session::Session;
use crate::session_manager::SessionManager;
use async_mutex::{Mutex, MutexGuard};
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::machine_check_in_server::MachineCheckIn;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::{
    machine_request, CheckInRequest, Hash, MerkleTreeProof, Void,
};
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine_manager::machine_manager_server::MachineManager;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine_manager::session_run_response::RunOneof;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine_manager::session_step_request::StepParamsOneof;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine_manager::{
    EndSessionRequest, NewSessionRequest, SessionGetProofRequest, SessionReadMemoryRequest,
    SessionReadMemoryResponse, SessionRunProgress, SessionRunRequest, SessionRunResponse,
    SessionRunResult, SessionStepRequest, SessionStepResponse, SessionStoreRequest,
    SessionWriteMemoryRequest,
};
use session::SessionRequest;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub const CARTESI_BIN_PATH: &str = "CARTESI_BIN_PATH";
pub const CARTESI_IMAGE_PATH: &str = "CARTESI_IMAGE_PATH";

/// Service that implements check in
pub struct ManagerCheckinService {
    server_manager: Arc<Mutex<dyn ServerManager>>,
}

impl ManagerCheckinService {
    pub fn new(server_manager: Arc<Mutex<dyn ServerManager>>) -> Self {
        ManagerCheckinService { server_manager }
    }
}

#[tonic::async_trait]
impl MachineCheckIn for ManagerCheckinService {
    /// Check in grpc api implementation
    async fn check_in(
        &self,
        request: Request<CheckInRequest>,
    ) -> Result<tonic::Response<Void>, tonic::Status> {
        let request = request.into_inner();
        log::debug!(
            "session id: {} got a check in request, address={} ",
            &request.session_id,
            &request.address
        );
        let mut server_manager = self.server_manager.lock().await;
        server_manager.set_server_check_in_status(&request.session_id, true, &request.address);
        Ok(Response::new(Void {}))
    }
}

/// Service that implements Machine Manager grpc api
pub struct MachineManagerService {
    session_manager: Arc<Mutex<dyn SessionManager>>,
}

impl MachineManagerService {
    pub fn new(session_manager: Arc<Mutex<dyn SessionManager>>) -> Self {
        MachineManagerService { session_manager }
    }

    /// Check if session current request is same as pending request and return error.
    /// Otherwise, set pending request as current request
    fn check_and_set_new_request(
        session: &mut MutexGuard<Session>,
        request_info: &SessionRequest,
    ) -> Result<(), Status> {
        if let Some(current_request) = session.get_current_request() {
            if current_request == request_info {
                log::debug!(
                    "session id={} got same operation request, operation already in progress",
                    &request_info.id
                );
                Err(tonic::Status::already_exists(
                    "operation already in progress",
                ))
            } else {
                // todo review behaviour
                // Do nothing, call will hang on mutex waiting to be executed
                Ok(())
            }
        } else {
            // No request is currently processed, set pending request as current
            log::debug!("session id={} no request is currently processed, set pending request of type {} as current", &request_info.id, &request_info.r#type);
            session.set_current_request(request_info);
            Ok(())
        }
    }

    /// If error string matches pattern, deduce tonic error type to be invalid argument. Otherwise,
    /// resulting type is internal error
    fn deduce_tonic_error_type(error_str: &str, pattern: &str, message: &str) -> Status {
        return if let Some(_) = error_str.find(pattern) {
            tonic::Status::invalid_argument(message)
        } else {
            tonic::Status::internal(message)
        };
    }

    /// Handling of the pending session run job request, depending on the current session job execution status.
    async fn check_and_set_run_request(
        session_mut: Arc<Mutex<Session>>,
        request_info: &SessionRequest,
    ) -> Result<Option<Response<SessionRunResponse>>, Status> {
        let mut session = session_mut.lock().await;
        if let Some(current_request) = session.get_current_request() {
            if current_request == request_info {
                // Request is the same as session current request, return progress or result
                match session.get_job_progress(&request_info.id).await {
                    Ok((progress, hashes, summaries)) => {
                        let response;
                        if progress.halted && progress.progress < 100 {
                            //Machine halted before reaching the final requested cycle
                            response = SessionRunResponse {
                                run_oneof: Some(RunOneof::Result(SessionRunResult {
                                    hashes: hashes.iter().map(cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::Hash::from).collect(),
                                    summaries
                                })),
                            };
                            MachineManagerService::clear_request(&mut session);
                        } else if progress.progress < 100 {
                            response = SessionRunResponse {
                                run_oneof: Some(RunOneof::Progress(SessionRunProgress {
                                    cycle: progress.cycle,
                                    progress: progress.progress,
                                    updated_at: progress.updated_at,
                                    application_progress: progress.application_progress,
                                })),
                            };
                        } else {
                            response = SessionRunResponse {
                                run_oneof: Some(RunOneof::Result(SessionRunResult {
                                    hashes: hashes.iter().map(cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::Hash::from).collect(),
                                    summaries
                                })),
                            };
                            MachineManagerService::clear_request(&mut session);
                        }
                        Ok(Some(Response::new(response)))
                    }
                    Err(err) => {
                        let error_msg =
                            format!("unable to get execution progress: {}", err.to_string());
                        log::error!("{}", &error_msg);
                        return Err(tonic::Status::invalid_argument(&error_msg));
                    }
                }
            } else {
                // New request different then currently processed request, abort
                // current request and set pending request as current
                log::debug!(
                    "abort current run task for session {}, starting new job",
                    session.get_id()
                );
                MachineManagerService::clear_request(&mut session);
                session.set_current_request(request_info);
                Ok(None)
            }
        } else {
            // No request is currently processed, set pending request as current
            log::debug!("no request is currently processed, set pending request as current");
            session.set_current_request(request_info);
            Ok(None)
        }
    }

    /// Helper function to clear current request of the session
    fn clear_request(session: &mut MutexGuard<Session>) {
        session.clear_job();
        session.clear_request();
    }

    /// Helper function to find session by id in session manager. If not found
    /// return invalid argument error
    async fn find_session(&self, session_id: &str) -> Result<Arc<Mutex<Session>>, Status> {
        match self
            .session_manager
            .lock()
            .await
            .get_session(session_id)
            .await
        {
            Ok(session_mut) => Ok(Arc::clone(&session_mut)),
            Err(_err) => {
                return Err(tonic::Status::invalid_argument(format!(
                    "unknown session id {}",
                    session_id
                )));
            }
        }
    }
}

/// Implementation of the MachineManager service Protobuf API
#[tonic::async_trait]
impl MachineManager for MachineManagerService {
    /// Implementation of rpc NewSession (NewSessionRequest) returns (CartesiMachine.Hash)
    async fn new_session(
        &self,
        request: Request<NewSessionRequest>,
    ) -> Result<Response<Hash>, Status> {
        let request_info = SessionRequest::from(&request);
        let new_session_request = request.into_inner();
        let session_id = &new_session_request.session_id;
        let force = new_session_request.force; // Force creating a new session
        log::info!(
            "session id={} received new session request",
            &new_session_request.session_id
        );
        log::debug!(
            "New session request info, session id={}, {:#?}",
            &new_session_request.session_id,
            &request_info
        );

        match new_session_request.machine {
            Some(m_request) => {
                // Get runtime config from request
                let runtime_config = match m_request.runtime {
                    Some(rc) => grpc_cartesi_machine::MachineRuntimeConfig::from(&rc),
                    None => Default::default(),
                };
                return if let Some(machine_one_of) = &m_request.machine_oneof {
                    match machine_one_of {
                        machine_request::MachineOneof::Config(machine_config) => {
                            // Create session from configuration
                            let config = grpc_cartesi_machine::MachineConfig::from(machine_config);
                            log::debug!(
                                "creating session id={} from configuration {:#?}",
                                &session_id,
                                &config
                            );
                            let mut session_manager = self.session_manager.lock().await;
                            match session_manager
                                .create_session_from_config(
                                    session_id,
                                    &config,
                                    &runtime_config,
                                    &request_info,
                                    force,
                                )
                                .await
                            {
                                Ok(_session) => (),
                                Err(err) => {
                                    return if let Some(_) = err.to_string().find(&"already exists")
                                    {
                                        let error_message = format!(
                                            "error creating new session id={}. Session already exists",
                                            session_id
                                        );
                                        log::error!("{}", &error_message);
                                        Err(tonic::Status::invalid_argument(error_message))
                                    } else {
                                        let error_message = format!(
                                            "Error creating new session id={}. Details: '{}'",
                                            session_id,
                                            err.to_string()
                                        );
                                        log::error!("{}", &error_message);
                                        Err(tonic::Status::internal(error_message))
                                    }
                                }
                            }
                            // Retrieve session and return initial hash
                            match session_manager.get_session(session_id).await {
                                Ok(session_mut) => {
                                    match session_mut.lock().await.get_root_hash().await {
                                        Ok(hash) => Ok(Response::new(Hash::from(&hash))),
                                        Err(err) => {
                                            let error_message = format!(
                                                "session id={} error creating new session. Details: '{}'",
                                                &session_id,
                                                err.to_string()
                                            );
                                            log::error!("{}", &error_message);
                                            Err(tonic::Status::internal(error_message))
                                        }
                                    }
                                }
                                Err(err) => {
                                    let error_message = format!(
                                        "Error creating new session id={} - unable to get machine hash. Details: '{}'",
                                        &session_id,
                                        err.to_string()
                                    );
                                    log::error!("{}", &error_message);
                                    Err(tonic::Status::internal(error_message))
                                }
                            }
                        }
                        machine_request::MachineOneof::Directory(directory) => {
                            // Session creation from directory
                            log::debug!(
                                "creating session id={} with directory argument: {}",
                                &session_id,
                                directory
                            );
                            let mut session_manager = self.session_manager.lock().await;
                            match session_manager
                                .create_session_from_directory(
                                    session_id,
                                    directory,
                                    &runtime_config,
                                    &request_info,
                                    force,
                                )
                                .await
                            {
                                Ok(_session) => (),
                                Err(err) => {
                                    return if let Some(_) = err.to_string().find(&"already exists")
                                    {
                                        let error_message = format!(
                                            "error creating new session id={}. Session already exists",
                                            session_id
                                        );
                                        log::error!("{}", &error_message);
                                        Err(tonic::Status::invalid_argument(error_message))
                                    } else {
                                        let error_message = format!(
                                            "error creating new session id={}. Details: '{}'",
                                            session_id,
                                            err.to_string()
                                        );
                                        log::error!("{}", &error_message);
                                        Err(tonic::Status::internal(error_message))
                                    }
                                }
                            }
                            // Retrieve session and return initial hash
                            match session_manager.get_session(session_id).await {
                                Ok(session_mut) => {
                                    match session_mut.lock().await.get_root_hash().await {
                                        Ok(hash) => {
                                            log::info!(
                                                "new session request finished, returning hash {:?}",
                                                &hash
                                            );
                                            Ok(Response::new(Hash::from(&hash)))
                                        }
                                        Err(err) => {
                                            let error_message = format!(
                                                "Error creating new session id={}. Details: '{}'",
                                                &session_id,
                                                err.to_string()
                                            );
                                            log::error!("{}", &error_message);
                                            Err(tonic::Status::internal(error_message))
                                        }
                                    }
                                }
                                Err(err) => {
                                    let error_message = format!(
                                        "error creating new session id={} - unable to get machine hash. Details: '{}'",
                                        &session_id,
                                        err.to_string()
                                    );
                                    log::error!("{}", &error_message);
                                    Err(tonic::Status::internal(error_message))
                                }
                            }
                        }
                    }
                } else {
                    let error_message = format!(
                        "error creating new session id={} - missing argument",
                        &session_id
                    );
                    log::error!("{}", &error_message);
                    Err(tonic::Status::invalid_argument(error_message))
                };
            }
            None => {
                let error_message = format!(
                    "error creating new session id={} - missing argument",
                    &session_id
                );
                log::error!("{}", &error_message);
                return Err(tonic::Status::invalid_argument(error_message));
            }
        }
    }

    /// Implementation of rpc SessionRun (SessionRunRequest) returns (SessionRunResponse)
    async fn session_run(
        &self,
        request: Request<SessionRunRequest>,
    ) -> Result<Response<SessionRunResponse>, Status> {
        let request_info = SessionRequest::from(&request);
        let run_request = request.into_inner();
        log::info!(
            "session id={} received session run request, final_cycles={:?}",
            &run_request.session_id,
            &run_request.final_cycles
        );
        let session_mut = self.find_session(&run_request.session_id).await?;
        if run_request.final_cycles.is_empty() {
            let error_message = format!(
                "session id={} error running session - empty list of final cycles",
                &run_request.session_id
            );
            log::error!("{}", &error_message);
            return Err(tonic::Status::invalid_argument(error_message));
        }
        // Check if we got same run request and should return progress/result
        match MachineManagerService::check_and_set_run_request(
            Arc::clone(&session_mut),
            &request_info,
        )
        .await
        {
            Ok(progress_status) => {
                if let Some(progress) = progress_status {
                    return Ok(progress);
                }
            }
            Err(e) => return Err(e),
        };
        // Start new run job
        let run_result = match Session::run(
            Arc::clone(&session_mut),
            &request_info.id,
            &run_request.final_cycles,
        )
        .await
        {
            Ok(progress) => {
                //Job started, return initial progress
                Ok(SessionRunResponse {
                    run_oneof: Some(RunOneof::Progress(progress)),
                })
            }
            Err(err) => Err(err.to_string()),
        };

        match run_result {
            Ok(response) => {
                if let Some(RunOneof::Result { .. }) = &response.run_oneof {
                    //Clear current request and job task
                    let mut session = session_mut.lock().await;
                    MachineManagerService::clear_request(&mut session); //Clear current RUN request
                }
                log::info!(
                    "session_id='{}' run request processed successfully",
                    &run_request.session_id
                );
                Ok(Response::new(response))
            }
            Err(err_str) => {
                //Clear current request and job task
                let mut session = session_mut.lock().await;
                MachineManagerService::clear_request(&mut session); //Clear current RUN request
                log::error!("{}", &err_str);
                Err(tonic::Status::internal(err_str))
            }
        }
    }

    /// Implementation of rpc SessionStep (SessionStepRequest) returns (SessionStepResponse)
    async fn session_step(
        &self,
        request: Request<SessionStepRequest>,
    ) -> Result<Response<SessionStepResponse>, Status> {
        let request_info = SessionRequest::from(&request);
        let step_request = request.into_inner();
        log::info!(
            "session id={} received session step request, initial cycle: {}",
            &step_request.session_id,
            &step_request.initial_cycle
        );
        let session_mut = self.find_session(&step_request.session_id).await?;
        let mut session = session_mut.lock().await;
        match &step_request.step_params_oneof {
            Some(step_params_oneof) => match step_params_oneof {
                StepParamsOneof::StepParams(request) => {
                    MachineManagerService::check_and_set_new_request(&mut session, &request_info)?;
                    if let Some(log_type) = &request.log_type {
                        // Perform step
                        match session
                            .step(
                                step_request.initial_cycle,
                                &grpc_cartesi_machine::AccessLogType::from(log_type),
                                request.one_based,
                            )
                            .await
                        {
                            Ok(log) => {
                                let response = SessionStepResponse {
                                        log: Some(cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::AccessLog::from(&log))
                                    };
                                MachineManagerService::clear_request(&mut session);
                                log::info!(
                                    "session id={} step request executed successfully",
                                    &step_request.session_id
                                );
                                Ok(Response::new(response))
                            }
                            Err(err) => {
                                MachineManagerService::clear_request(&mut session);
                                let error_message = format!(
                                    "error executing session step for session id={}. Details:'{}'",
                                    &session.get_id(),
                                    err.to_string()
                                );
                                log::error!("{}", &error_message);
                                return Err(MachineManagerService::deduce_tonic_error_type(
                                    &err.to_string(),
                                    "unexpected session cycle, current cycle is",
                                    &error_message,
                                ));
                            }
                        }
                    } else {
                        let error_message = "step request invalid argument, missing log type";
                        log::error!("{}", &error_message);
                        return Err(tonic::Status::invalid_argument(error_message));
                    }
                }
            },
            None => {
                let error_message = "step request invalid argument, missing step params argument";
                log::error!("{}", &error_message);
                return Err(tonic::Status::invalid_argument(error_message));
            }
        }
    }

    /// Implementation of rpc SessionStore (SessionStoreRequest) returns (CartesiMachine.Void)
    async fn session_store(
        &self,
        request: Request<SessionStoreRequest>,
    ) -> Result<Response<cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::Void>, Status> {
        let request_info = SessionRequest::from(&request);
        let store_request = request.into_inner();
        log::info!(
            "received session store request, session id={}",
            &store_request.session_id
        );
        let session_mut = self.find_session(&store_request.session_id).await?;
        let mut session = session_mut.lock().await;
        match store_request.store {
            Some(st_req) => {
                MachineManagerService::check_and_set_new_request(&mut session, &request_info)?;
                match session.store(&st_req.directory).await {
                    Ok(_) => (),
                    Err(err) => {
                        MachineManagerService::clear_request(&mut session);
                        let error_message = err.to_string();
                        log::error!("{}", &error_message);
                        return Err(tonic::Status::internal(error_message));
                    }
                }
            }
            None => {
                let error_message =
                    "error execution session store request - missing store directory argument";
                log::error!("{}", &error_message);
                return Err(tonic::Status::invalid_argument(error_message));
            }
        }
        MachineManagerService::clear_request(&mut session);
        log::info!(
            "session id={} store request executed successfully",
            &store_request.session_id
        );
        Ok(Response::new(Void {}))
    }

    /// Implementation of rpc SessionReadMemory (SessionReadMemoryRequest) returns (SessionReadMemoryResponse)
    async fn session_read_memory(
        &self,
        request: Request<SessionReadMemoryRequest>,
    ) -> Result<Response<SessionReadMemoryResponse>, Status> {
        let request_info = SessionRequest::from(&request);
        let read_request = request.into_inner();
        log::info!(
            "received read memory request, session id={}, cycle: {}",
            &read_request.session_id,
            &read_request.cycle
        );
        let session_mut = self.find_session(&read_request.session_id).await?;
        let mut session = session_mut.lock().await;
        match &read_request.position {
            Some(position) => {
                log::debug!(
                    "executing read memory request for session {}, cycle: {} address {} length {}",
                    &read_request.session_id,
                    &read_request.cycle,
                    &position.address,
                    &position.length
                );
                MachineManagerService::check_and_set_new_request(&mut session, &request_info)?;
                match session
                    .read_mem(read_request.cycle, position.address, position.length)
                    .await
                {
                    Ok(data) => {
                        let response = SessionReadMemoryResponse{
                            read_content: Some(cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::ReadMemoryResponse{
                                data
                            })
                        };
                        MachineManagerService::clear_request(&mut session);
                        log::info!(
                            "session id={} read memory request executed successfully",
                            &read_request.session_id
                        );
                        Ok(Response::new(response))
                    }
                    Err(err) => {
                        MachineManagerService::clear_request(&mut session);
                        let error_message = format!(
                            "error executing session read memory for session id={}. Details:'{}'",
                            &session.get_id(),
                            err.to_string()
                        );
                        log::error!("{}", &error_message);
                        return Err(MachineManagerService::deduce_tonic_error_type(
                            &err.to_string(),
                            "unexpected session cycle, current cycle is",
                            &error_message,
                        ));
                    }
                }
            }
            None => {
                let error_message =
                    "error executing session read memory request - missing position argument";
                log::error!("{}", &error_message);
                return Err(tonic::Status::invalid_argument(error_message));
            }
        }
    }

    /// Implementation of rpc SessionWriteMemory (SessionWriteMemoryRequest) returns (CartesiMachine.Void)
    async fn session_write_memory(
        &self,
        request: Request<SessionWriteMemoryRequest>,
    ) -> Result<Response<cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::Void>, Status> {
        let request_info = SessionRequest::from(&request);
        let write_request = request.into_inner();
        log::info!(
            "received session write memory request, session id={}, cycle={}",
            &write_request.session_id,
            &write_request.cycle
        );
        let session_mut = self.find_session(&write_request.session_id).await?;
        let mut session = session_mut.lock().await;
        match write_request.position {
            Some(position) => {
                MachineManagerService::check_and_set_new_request(&mut session, &request_info)?;
                match session
                    .write_mem(write_request.cycle, position.address, position.data)
                    .await
                {
                    Ok(()) => {
                        MachineManagerService::clear_request(&mut session);
                        log::info!(
                            "session id={} write memory request executed successfully",
                            &write_request.session_id
                        );
                        Ok(Response::new(Void {}))
                    }
                    Err(err) => {
                        MachineManagerService::clear_request(&mut session);
                        let error_message = format!(
                            "Error executing session write memory for session id={}. Details:'{}'",
                            &session.get_id(),
                            err.to_string()
                        );
                        log::error!("{}", &error_message);
                        return Err(MachineManagerService::deduce_tonic_error_type(
                            &err.to_string(),
                            "unexpected session cycle, current cycle is",
                            &error_message,
                        ));
                    }
                }
            }
            None => {
                let error_message =
                    "Error executing session write memory request - missing position argument";
                log::error!("{}", &error_message);
                return Err(tonic::Status::invalid_argument(error_message));
            }
        }
    }

    /// Implementation of rpc SessionGetProof (SessionGetProofRequest) returns (CartesiMachine.MerkleTreeProof)
    async fn session_get_proof(
        &self,
        request: Request<SessionGetProofRequest>,
    ) -> Result<Response<MerkleTreeProof>, Status> {
        let request_info = SessionRequest::from(&request);
        let proof_request = request.into_inner();
        log::info!(
            "received session get proof request, session id={}, cycle={}",
            &proof_request.session_id,
            &proof_request.cycle
        );

        let session_mut = self.find_session(&proof_request.session_id).await?;
        let mut session = session_mut.lock().await;
        match proof_request.target {
            Some(target) => {
                MachineManagerService::check_and_set_new_request(&mut session, &request_info)?;
                match session
                    .get_proof(proof_request.cycle, target.address, target.log2_size)
                    .await
                {
                    Ok(result) => {
                        MachineManagerService::clear_request(&mut session);
                        log::info!(
                            "session id={} get proof request executed successfully",
                            &proof_request.session_id
                        );
                        Ok(Response::new(MerkleTreeProof::from(&result)))
                    }
                    Err(err) => {
                        MachineManagerService::clear_request(&mut session);
                        let error_message = &format!(
                            "error executing session get proof for session id={}. Details:'{}'",
                            &session.get_id(),
                            err.to_string()
                        );
                        log::error!("{}", &error_message);
                        return Err(MachineManagerService::deduce_tonic_error_type(
                            &err.to_string(),
                            "unexpected session cycle, current cycle is",
                            &error_message,
                        ));
                    }
                }
            }
            None => {
                let error_message =
                    "error executing session get proof request - missing target argument";
                log::error!("{}", &error_message);
                return Err(tonic::Status::invalid_argument(error_message));
            }
        }
    }

    /// Implementation of rpc EndSession (EndSessionRequest) returns (CartesiMachine.Void)
    async fn end_session(
        &self,
        request: Request<EndSessionRequest>,
    ) -> Result<Response<cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::Void>, Status> {
        let end_request = request.into_inner();
        log::info!(
            "received end session request, session id={}",
            &end_request.session_id
        );
        let mut session_manager = self.session_manager.lock().await;
        match session_manager.close_session(&end_request.session_id).await {
            Ok(()) => {
                log::info!(
                    "end session id={} request executed successfully",
                    &end_request.session_id
                );
                Ok(Response::new(Void {}))
            }
            Err(err) => {
                let error_message = format!(
                    "Error ending session id={}. Details: '{}'",
                    &end_request.session_id,
                    err.to_string()
                );
                log::error!("{}", &error_message);
                return Err(tonic::Status::internal(error_message));
            }
        }
    }
}
