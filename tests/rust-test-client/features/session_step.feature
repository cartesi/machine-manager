# Copyright 2023 Cartesi Pte. Ltd.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use
# this file except in compliance with the License. You may obtain a copy of the
# License at http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software distributed
# under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
# CONDITIONS OF ANY KIND, either express or implied. See the License for the
# specific language governing permissions and limitations under the License.

Feature: SessionStep feature

    Scenario Outline: steps with different cycles

        # - Step with initial cycle = 0 should happen on a new machine.
        # - Step with initial cycle < machine cycle and initial cycle > snapshot cycle should force machine rollback.
        # - Step with initial cycle < machine cycle and initial cycle < snapshot cycle should force machine recreation.
        # - Step with initial cycle > machine cycle should not require any special effort.
        # - Step with initial cycle = machine cycle, should not even require making a dummy run to get into machine
        #   cycle = initial cycle.

        Given machine manager server is up
        And a machine manager server with a machine executed for <cycle> final cycles and <ucycle> final ucycles
        When the machine manager server asks machine to step on initial cycle <cycle> and initial ucycle <ucycle>
        Then machine manager server returns correct session cycle <result_cycle> and ucycle <result_ucycle>
        And server returns correct access log

        Examples:
            | cycle | ucycle | result_cycle | result_ucycle |
            |   1   |   1    |      1       |      2        |
            |   21  |   101  |      21      |      102      |
            |   35  |   250  |      35      |      251      |
            |   30  |   500  |      30      |      501      |

    Scenario Outline: step on invalid cycle

        # For rust machine manager:
        # For operations ReadMemory/WriteMemory/Step/GetProof, in case where cycle argument is not equal to current
        # session cycle argument, return error. SessionRun request should be used to run machine to particular cycle.

        Given machine manager server is up
        And a machine manager server with a machine executed for 20 final cycles and 300 final ucycles
        When the machine manager server asks machine to step on initial cycle 15 and initial ucycle 300
        Then machine manager server returns an InvalidArgument error

    Scenario Outline: step on invalid ucycle

        # For rust machine manager:
        # For operations ReadMemory/WriteMemory/Step/GetProof, in case where cycle argument is not equal to current
        # session ucycle argument, return error. SessionRun request should be used to run machine to particular ucycle.

        Given machine manager server is up
        And a machine manager server with a machine executed for 20 final cycles and 300 final ucycles
        When the machine manager server asks machine to step on initial cycle 20 and initial ucycle 150
        Then machine manager server returns an InvalidArgument error