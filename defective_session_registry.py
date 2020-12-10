"""
Copyright 2019 Cartesi Pte. Ltd.

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
"""

import session_registry
import utils

LOGGER = utils.get_new_logger("!!!!!DEFECTIVE!!!!!_" + __name__ + "_!!!!!DEFECTIVE!!!!!")
LOGGER = utils.configure_log(LOGGER)
MAX_CYCLE = 10000001

class SessionRegistryManager(session_registry.SessionRegistryManager):

    def run_session(self, session_id, final_cycles):

        #Saturating required cycles on MAX_CYCLE
        modified_cycles = []

        for cycle in final_cycles:
            if cycle >= MAX_CYCLE:
                modified_cycles.append(MAX_CYCLE)
            else:
                modified_cycles.append(int(cycle))

        LOGGER.debug("Executing defective run for session '{}'\nDesired cycles: {}\nUsed cycles: {}".format(session_id, final_cycles, modified_cycles))

        #Executing run over the desired modified cycles
        session_run_result = super().run_session(session_id, modified_cycles)

        #Modify response to mask that the requested cycles were saturated on MAX_CYCLE
        for i,summary in enumerate(session_run_result.result.summaries):
            summary.mcycle=int(final_cycles[i])

        LOGGER.debug("Finished executing defective run for session '{}'\nDesired cycles: {}\nUsed cycles: {}".format(session_id, final_cycles, modified_cycles))

        return session_run_result

    def step_session(self, session_id, initial_cycle, step_params):

        #Modifying cycle to saturate on MAX_CYCLE - 1
        modified_cycle = int(initial_cycle)

        if (modified_cycle >= MAX_CYCLE):
            modified_cycle = MAX_CYCLE - 1

        LOGGER.debug("Executing defective step for session '{}'\nDesired cycle: {}\nUsed cycle: {}".format(session_id, initial_cycle, modified_cycle))

        #Running and returning as response doesn't contain cycle numbers that need to be reset to mask defect
        session_step_result = super().step_session(session_id, modified_cycle, step_params)

        LOGGER.debug("Finished executing defective step for session '{}'\nDesired cycle: {}\nUsed cycle: {}".format(session_id, initial_cycle, modified_cycle))

        return session_step_result
