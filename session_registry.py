from threading import Lock, Event
import utils

LOGGER = utils.get_new_logger(__name__)
LOGGER = utils.configure_log(LOGGER)

class AddressException(Exception):
    pass

class SessionIdException(Exception):
    pass

class SessionRegistryManager:
    
    def __init__(self):
        self.global_lock = Lock()
        self.registry = {}
        
    def register_session(self, session_id):        
        #Acquiring global lock and releasing it when completed
        LOGGER.debug("Acquiring session registry global lock")
        with self.global_lock:
            LOGGER.debug("Lock acquired")
            if session_id in self.registry.keys():
                #Session id already in use
                err_msg = "Trying to register a session with an id that already exists: {}".format(session_id)
                LOGGER.error(err_msg)
                raise SessionIdException(err_msg)
            else:
                #Registering new session
                self.registry[session_id] = CartesiSession(session_id)
                LOGGER.info("New session registered: {}".format(session_id))                
            
    def wait_for_session_address_communication(self, session_id):
        #TODO, throw exception if session_id doesn't exist
        LOGGER.debug("Waiting for session address communication: {}".format(session_id))
        self.registry[session_id].address_set_event.wait()
        LOGGER.debug("Address for session {} communicated: {}".format(session_id, self.registry[session_id].address))
        
    def register_address_for_session(self, session_id, address):
        
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
            
        LOGGER.debug("Acquiring session registry global lock")
        #Acquiring lock to write on session registry
        with self.global_lock:
            LOGGER.debug("Session registry global lock acquired")
            #Registering address and notifying that it was set
            self.registry[session_id].address = address
            self.registry[session_id].address_set_event.set()
            LOGGER.debug("Address for session {} set: {}".format(session_id, address))
    
    def create_new_cartesi_machine_server(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (self.registry[session_id].address):
            raise AddressException("Address already set for server with session_id '{}'".format(session_id))
        LOGGER.debug("Acquiring session '{}' registry lock".format(session_id))
        with self.registry[session_id].lock:
            LOGGER.debug("Lock acquired for session '{}', creating new cartesi machine server".format(session_id))
            utils.new_cartesi_machine_server(session_id)
            LOGGER.debug("Server created for session '{}'".format(session_id))
    
    def create_machine(self, session_id, machine_req):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))
        LOGGER.debug("Acquiring session '{}' registry lock".format(session_id))       
        with self.registry[session_id].lock:
            LOGGER.debug("Issuing server to create a new machine for session '{}'".format(session_id))
            utils.new_machine(session_id, self.registry[session_id].address, machine_req)
            LOGGER.debug("Executed creating a new machine for session '{}'".format(session_id))
            
    def get_machine_root_hash(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))
        LOGGER.debug("Acquiring session '{}' registry lock".format(session_id))       
        with self.registry[session_id].lock:
            LOGGER.debug("Issuing server to get machine root hash for session '{}'".format(session_id))
            root_hash = utils.get_machine_hash(session_id, self.registry[session_id].address)
            LOGGER.debug("Executed getting machine root hash for session '{}'".format(session_id))
            return root_hash
        
    def snapshot_machine(self, session_id):
        if (session_id not in self.registry.keys()):
            raise SessionIdException("No session in registry with provided session_id: {}".format(session_id))
        if (not self.registry[session_id].address):
            raise AddressException("Address not set for server with session_id '{}'. Check if machine server was created correctly".format(session_id))
        LOGGER.debug("Acquiring session '{}' registry lock".format(session_id))       
        with self.registry[session_id].lock:
            LOGGER.debug("Issuing server to create machine snapshot for session '{}'".format(session_id))
            utils.create_machine_snapshot(session_id, self.registry[session_id].address)
            LOGGER.debug("Executed creating machine snapshot for session '{}'".format(session_id))
            LOGGER.debug("Acquiring session registry global lock")
            #Acquiring lock to write on session registry
            with self.global_lock:
                LOGGER.debug("Session registry global lock acquired")
                self.registry[session_id].snapshot_time = self.registry[session_id].time
                LOGGER.debug("Updated snapshot time of session {} to {}".format(session_id, self.registry[session_id].time))
            
class CartesiSession:
    
    def __init__(self, session_id):
        self.id = session_id
        self.lock = Lock()
        self.address = None
        self.address_set_event = Event()
        self.time = 0
        self.snapshot_time = None
    
            
            
            