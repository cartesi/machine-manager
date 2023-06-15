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

Feature: SessionStoreLoad feature

    Scenario: perform regular store-load
        Given machine manager server is up
        And a pristine machine manager server session
        When asking machine manager server to store the machine in a directory /stored_machine
        Then machine manager server is able to load machine from this directory correctly
        And machine manager server is able to execute this machine for 60 cycles and 0 ucycles
        And server returns correct machine hashes
