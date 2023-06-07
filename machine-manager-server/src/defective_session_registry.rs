// Copyright 2023 Cartesi Pte. Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use machine_manager_server::session::Session;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::{
    machine_request, Hash, MerkleTreeProof, Void,
};
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine_manager::machine_manager_server::MachineManager;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine_manager::session_run_response::RunOneof;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine_manager::session_step_request::StepParamsOneof;
use cartesi_grpc_interfaces::grpc_stubs::cartesi_machine_manager::{
    EndSessionRequest, NewSessionRequest, SessionGetProofRequest, SessionReadMemoryRequest,
    SessionReadMemoryResponse, SessionRunRequest, SessionRunResponse,
    SessionStepRequest, SessionStepResponse, SessionStoreRequest,
    SessionWriteMemoryRequest, SessionReplaceMemoryRangeRequest,
};
use machine_manager_server::session::SessionRequest;
use std::result;
use std::sync::Arc;
use tonic::{Request, Response, Status};
extern crate machine_manager_server;
use machine_manager_server::MachineManagerService;


pub struct MachineManagerServiceDefective{
    pub machine_manager_service: MachineManagerService
}

impl MachineManagerServiceDefective {
    pub fn new(machine_manager_service: MachineManagerService) -> Self {
        MachineManagerServiceDefective { machine_manager_service }
    }
}

/// Implementation of the MachineManager service Protobuf API
#[tonic::async_trait]
impl MachineManager for MachineManagerServiceDefective {
    /// Implementation of rpc NewSession (NewSessionRequest) returns (CartesiMachine.Hash)
    async fn new_session(
        &self,
        request: Request<NewSessionRequest>,
    ) -> Result<Response<Hash>, Status> {

        if self.machine_manager_service.shutting_down().await{
            let error_message = String::from("Server is shutting down, not accepting new requests");
            return Err(tonic::Status::unavailable(error_message));
        }

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
                            let mut session_manager = self.machine_manager_service.session_manager.lock().await;
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
                            let mut session_manager = self.machine_manager_service.session_manager.lock().await;
                            match session_manager
                                .create_session_from_directory(
                                    session_id,
                                    &directory,
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

        if self.machine_manager_service.shutting_down().await{
            let error_message = String::from("Server is shutting down, not accepting new requests");
            return Err(tonic::Status::unavailable(error_message));
        }

        let request_info = SessionRequest::from(&request);
        let run_request = request.into_inner();
        log::info!(
            "session id={} received session run request, final_cycles={:?}",
            &run_request.session_id,
            &run_request.final_cycles
        );
        let session_mut = self.machine_manager_service.find_session(&run_request.session_id).await?;
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
        let run_result = match Session::run_defective(
            Arc::clone(&session_mut),
            &request_info.id,
            &run_request.final_cycles,
            &run_request.final_ucycles,
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

        if self.machine_manager_service.shutting_down().await{
            let error_message = String::from("Server is shutting down, not accepting new requests");
            return Err(tonic::Status::unavailable(error_message));
        }
        
        let request_info = SessionRequest::from(&request);
        let step_request = request.into_inner();
        log::info!(
            "session id={} received session step request, initial cycle: {}",
            &step_request.session_id,
            &step_request.initial_cycle
        );
        let session_mut = self.machine_manager_service.find_session(&step_request.session_id).await?;
        let mut session = session_mut.lock().await;
        match &step_request.step_params_oneof {
            Some(step_params_oneof) => match step_params_oneof {
                StepParamsOneof::StepParams(request) => {
                    MachineManagerService::check_and_set_new_request(&mut session, &request_info)?;
                    if let Some(log_type) = &request.log_type {
                        // Perform step
                        match session
                            .step_defective(
                                step_request.initial_cycle,
                                step_request.initial_ucycle,
                                &grpc_cartesi_machine::AccessLogType::from(log_type),
                                request.one_based,
                            )
                            .await
                        {
                            Ok(result) => {
                                let response = SessionStepResponse {
                                        log: Some(cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::AccessLog::from(&result.2)),
                                        cycle: result.0,
                                        ucycle: result.1,
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
                                    "unexpected requested",
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
        let session_mut = self.machine_manager_service.find_session(&store_request.session_id).await?;
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
        let session_mut = self.machine_manager_service.find_session(&read_request.session_id).await?;
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
                    .read_mem(read_request.cycle, read_request.ucycle, position.address, position.length)
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
                            "unexpected requested",
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

    async fn session_replace_memory_range(
        &self,
        request: Request<SessionReplaceMemoryRangeRequest>,
    ) -> Result<Response<cartesi_grpc_interfaces::grpc_stubs::cartesi_machine::Void>, Status> {
        let request_info = SessionRequest::from(&request);
        let replace_request = request.into_inner();
        log::info!(
            "received session replace memory range request, session id={}, cycle={}",
            &replace_request.session_id,
            &replace_request.cycle
        );
        let session_mut = self.machine_manager_service.find_session(&replace_request.session_id).await?;
        let mut session = session_mut.lock().await;
        match replace_request.range {
             Some(range) => {
               MachineManagerService::check_and_set_new_request(&mut session, &request_info)?;

              match session
              .replace_memory_range(replace_request.cycle, replace_request.ucycle, &range)
                    .await
                {
                    Ok(()) => {
                        MachineManagerService::clear_request(&mut session);
                        log::info!(
                            "session id={} replace memory range request executed successfully",
                            &replace_request.session_id
                        );
                        Ok(Response::new(Void {}))
                    }
                    Err(err) => {
                        MachineManagerService::clear_request(&mut session);
                        let error_message = format!(
                            "Error executing session replace memory range for session id={}. Details:'{}'",
                            &session.get_id(),
                            err.to_string()
                        );
                        log::error!("{}", &error_message);
                        return Err(MachineManagerService::deduce_tonic_error_type(
                            &err.to_string(),
                            "unexpected requested",
                            &error_message,
                        ));
                    }
                }
            }
            None => {
                let error_message =
                    "Error executing session replac memory range request - missing range argument";
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
        let session_mut = self.machine_manager_service.find_session(&write_request.session_id).await?;
        let mut session = session_mut.lock().await;
        match write_request.position {
            Some(position) => {
                MachineManagerService::check_and_set_new_request(&mut session, &request_info)?;
                match session
                    .write_mem(write_request.cycle, write_request.ucycle, position.address, position.data)
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
                            "unexpected requested",
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

        let session_mut = self.machine_manager_service.find_session(&proof_request.session_id).await?;
        let mut session = session_mut.lock().await;
        match proof_request.target {
            Some(target) => {
                MachineManagerService::check_and_set_new_request(&mut session, &request_info)?;
                match session
                    .get_proof(proof_request.cycle, proof_request.ucycle, target.address, target.log2_size)
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
                            "unexpected requested",
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
        let mut session_manager = self.machine_manager_service.session_manager.lock().await;
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
