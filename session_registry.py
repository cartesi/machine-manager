from threading import Lock, Event
import utils
import time

WAIT_SERVER_TIME = 2

LOGGER = utils.get_new_logger(__name__)
LOGGER = utils.configure_log(LOGGER)

class AddressException(Exception):
    pass

class SessionIdException(Exception):
    pass

class RollbackException(Exception):
    pass

class SessionRegistryManager:
    
    def __init__(self):
        self.global_lock = Lock()
        self.registry = {}
        
    def new_session(self, session_id, machine_req):
        #Registering new session
        self.register_session(session_id)
        
        LOGGER.debug("Acquiring lock for session {}".format(session_id))
        with self.registry[session_id].lock:
            #Instantiate new cartesi machine server
            self.create_new_cartesi_machine_server(session_id)
        
            #Wait for the new server to communicate it's listening address
            self.wait_for_session_address_communication(session_id)
        
            #Communication received, create new cartesi machine
            self.create_machine(session_id, machine_req)
        
            #calculate cartesi machine initial hash
            initial_hash = self.get_machine_root_hash(session_id)
        
            #Create snapshot
            self.snapshot_machine(session_id)
            
            #Wait for the server to communicate it's listening address after finishing the snapshot
            self.wait_for_session_address_communication(session_id)
            
        LOGGER.debug("Released lock for session {}".format(session_id))
        
        return initial_hash
    
    def run_session(self, session_id, times):
        
        summaries = []
        hashes = []
        
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))
            
        LOGGER.debug("Acquiring lock for session {}".format(session_id))
        with self.registry[session_id].lock:
            
            first_t = times.pop(0)
            
            #Checking machine time is after first required time
            if (self.registry[session_id].time > first_t):            
                #it is, checking if snapshot time is before or after required time
                if (self.registry[session_id].snapshot_time < first_t):                
                    #It is, rolling back
                    self.rollback_machine(session_id)
                else:
                    #It isn't, recreating machine from scratch
                    self.recreate_machine(session_id)
                    
            #Make execute and make machine snapshot
            summaries.append(self.run_and_update_registry_time(session_id, first_t))
            self.snapshot_machine(session_id)
            
            #Getting hash
            hashes.append(self.get_machine_root_hash(session_id))
            
            #Executing additional runs on given times
            for t in times:
                summaries.append(self.run_and_update_registry_time(session_id, t))
                hashes.append(self.get_machine_root_hash(session_id))
                
            #Returning SessionRunResult
            return utils.make_session_run_result(summaries, hashes)
        
        
        
    
    """
    Here starts the "internal" API, use the methods bellow taking the right precautions such as holding a lock a session
    """
    
    
    
    
    def register_session(self, session_id):        
        #Acquiring global lock and releasing it when completed
        LOGGER.debug("Acquiring session registry global lock")
        with self.global_lock:
            LOGGER.debug("Lock acquired")
            if session_id in self.registry.keys():
                #Session id already in use
                err_msg = "Trying to register a session with a session_id that already exists: {}".format(session_id)
                LOGGER.error(err_msg)
                raise SessionIdException(err_msg)
            else:
                #Registering new session
                self.registry[session_id] = CartesiSession(session_id)
                LOGGER.info("New session registered: {}".format(session_id))                
            
    def wait_for_session_address_communication(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id '{}'".format(session_id))
            
        LOGGER.debug("Waiting for session address communication for session_id '{}'".format(session_id))
        self.registry[session_id].address_set_event.wait()
        LOGGER.debug("Address for session_id '{}' communicated: {}".format(session_id, self.registry[session_id].address))
        #LOGGER.debug("Sleeping a bit")
        #time.sleep(WAIT_SERVER_TIME)
        #LOGGER.debug("Sleep is over")
        
    def register_address_for_session(self, session_id, address):
        
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id '{}'".format(session_id))
            
        LOGGER.debug("Acquiring session registry global lock")
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            #Registering address and notifying that it was set
            self.registry[session_id].address = address
            self.registry[session_id].address_set_event.set()
            LOGGER.debug("Address for session '{}' set to {}".format(session_id, address))
    
    def create_new_cartesi_machine_server(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id '{}'".format(session_id))
        if (self.registry[session_id].address):
            raise AddressException("Address already set for server with session_id '{}'".format(session_id))
            
        LOGGER.debug("Creating new cartesi machine server for session_id '{}'".format(session_id))
        utils.new_cartesi_machine_server(session_id)
        LOGGER.debug("Server created for session '{}'".format(session_id))
    
    def create_machine(self, session_id, machine_req):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))
            
        LOGGER.debug("Issuing server to create a new machine for session '{}'".format(session_id))
        utils.new_machine(session_id, self.registry[session_id].address, machine_req)
        LOGGER.debug("Executed creating a new machine for session '{}'".format(session_id))
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            self.registry[session_id].creation_machine_req = machine_req
            LOGGER.debug("Saved on registry machine request used to create session '{}'".format(session_id))        
            
    def get_machine_root_hash(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))
            
        LOGGER.debug("Issuing server to get machine root hash for session '{}'".format(session_id))
        root_hash = utils.get_machine_hash(session_id, self.registry[session_id].address)
        LOGGER.debug("Executed getting machine root hash for session '{}'".format(session_id))
        return root_hash
        
    def snapshot_machine(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))
            
        LOGGER.debug("Issuing server to create machine snapshot for session '{}'".format(session_id))
        utils.create_machine_snapshot(session_id, self.registry[session_id].address)
        LOGGER.debug("Executed creating machine snapshot for session '{}'".format(session_id))
        LOGGER.debug("Acquiring session registry global lock")
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            self.registry[session_id].snapshot_time = self.registry[session_id].time
            LOGGER.debug("Updated snapshot time of session '{}' to {}".format(session_id, self.registry[session_id].time))
        LOGGER.debug("Sleeping a bit")
        time.sleep(WAIT_SERVER_TIME)
        LOGGER.debug("Sleep is over")
            
    def rollback_machine(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))
        if (not self.registry[session_id].snapshot_time):
            raise RollbackException("There is no snapshot to rollback to for the cartesi machine with session_id '{}'".format(session_id))
            
        LOGGER.debug("Issuing server to rollback machine for session '{}'".format(session_id))
        utils.rollback_machine(session_id, self.registry[session_id].address)
        LOGGER.debug("Executed rollingback machine for session '{}'".format(session_id))
        LOGGER.debug("Acquiring session registry global lock")
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            self.registry[session_id].time = self.registry[session_id].snapshot_time
            self.registry[session_id].snapshot_time = None
            LOGGER.debug("Updated time of session '{}' to {} and cleared snapshot time".format(session_id, self.registry[session_id].time))
        LOGGER.debug("Sleeping a bit")
        time.sleep(WAIT_SERVER_TIME)
        LOGGER.debug("Sleep is over")
            
    def recreate_machine(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
            
        #Shutting down old server if any
        if (self.registry[session_id].address):    
            utils.shutdown_cartesi_machine_server(session_id, self.registry[session_id].address)
            
        LOGGER.debug("Acquiring session registry global lock")
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            #Cleaning old server session data
            self.registry[session_id].address = None
            self.registry[session_id].time = 0
            self.registry[session_id].snapshot_time = None
            self.registry[session_id].address_set_event = Event()
        LOGGER.debug("Cleaned old server session data for session '{}'".format(session_id))
        
        #Instantiate new cartesi machine server
        self.create_new_cartesi_machine_server(session_id)
        
        #Wait for the new server to communicate it's listening address
        self.wait_for_session_address_communication(session_id)
        
        #Communication received, create new cartesi machine using saved parameters
        self.create_machine(session_id, self.registry[session_id].creation_machine_req)        
            
    def run_and_update_registry_time(self, session_id, t):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))
            
        #Running cartesi machine
        result = utils.run_machine(session_id, self.registry[session_id].address, t)
        
        #Updating cartesi session time
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            self.registry[session_id].time = t
            
        LOGGER.debug("Updated time of session '{}' to {}".format(session_id, self.registry[session_id].time))
            
        return result
    
            
class CartesiSession:
    
    def __init__(self, session_id):
        self.id = session_id
        self.lock = Lock()
        self.address = None
        self.address_set_event = Event()
        self.time = 0
        self.snapshot_time = None
        self.creation_machine_req = None
    
            
            
            