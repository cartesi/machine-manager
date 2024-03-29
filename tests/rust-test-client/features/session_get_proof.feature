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

Feature: SessionGetProof feature

    Scenario Outline: asking for proofs with different parameters
        Given machine manager server is up
        And a machine manager server with a machine executed for <cycle> final cycles and <ucycle> final ucycles
        When the machine manager server asks machine for proof on cycle <cycle> and ucycle <ucycle> for address <address> with log2_size <size>
        Then server returns correct proof

        Examples:
            | cycle | ucycle |  address  | size |
            |  30   |   60   |    288    |  3   |
            |  30   |   60   |    288    |  4   |

    Scenario Outline: asking for proof on invalid cycle

        Given machine manager server is up
        And a machine manager server with a machine executed for 20 final cycles and 15 final ucycles
        When the machine manager server asks machine for proof on cycle <cycle> and ucycle 15 for address <address> with log2_size <size>
        Then machine manager server returns an InvalidArgument error

        Examples:
            | cycle | address| size |
            |   5   |   288  |  3   |

     Scenario Outline: asking for proof on invalid cycle

        Given machine manager server is up
        And a machine manager server with a machine executed for 20 final cycles and 15 final ucycles
        When the machine manager server asks machine for proof on cycle 20 and ucycle <ucycle> for address <address> with log2_size <size>
        Then machine manager server returns an InvalidArgument error

        Examples:
            | ucycle | address| size |
            |   5    |   288  |  3   |
