import session_registry
import utils

LOGGER = utils.get_new_logger("!!!!!DEFECTIVE!!!!!_" + __name__ + "_!!!!!DEFECTIVE!!!!!")
LOGGER = utils.configure_log(LOGGER)
MAX_CYCLE = 15

class SessionRegistryManager(session_registry.SessionRegistryManager):   
    
    def run_session(self, session_id, final_cycles):                
        
        #Saturating required cycles on MAX_CYCLE
        modified_cycles = []
        
        for cycle in final_cycles:
            if cycle >= MAX_CYCLE:
                modified_cycles.append(MAX_CYCLE)
            else:
                modified_cycles.append(int(cycle))
                
        LOGGER.debug("Executing defective run for session '{}'\nDesired cycles: {}\nUSed cycles: {}".format(session_id, final_cycles, modified_cycles))
        
        #Executing run over the desired modified cycles
        session_run_result = super().run_session(session_id, modified_cycles)
        
        #Modify response to mask that the requested cycles were saturated on MAX_CYCLE
        for i,summary in enumerate(session_run_result.summaries): 
            summary.mcycle=int(final_cycles[i]) 
            
        LOGGER.debug("Finished executing defective run for session '{}'\nDesired cycles: {}\nUSed cycles: {}".format(session_id, final_cycles, modified_cycles))

        return session_run_result
        
    def step_session(self, session_id, initial_cycle):
        
        #Modifying cycle to saturate on MAX_CYCLE - 1
        modified_cycle = int(initial_cycle)
        
        if (modified_cycle >= MAX_CYCLE):
            modified_cycle = MAX_CYCLE - 1
        
        LOGGER.debug("Executing defective step for session '{}'\nDesired cycle: {}\nUsed cycle: {}".format(session_id, initial_cycle, modified_cycle))
        
        #Running and returning as response doesn't contain cycle numbers that need to be reset to mask defect
        session_step_result = super().step_session(session_id, modified_cycle)        
        
        LOGGER.debug("Finished executing defective step for session '{}'\nDesired cycle: {}\nUsed cycle: {}".format(session_id, initial_cycle, modified_cycle))
        
        return session_step_result
