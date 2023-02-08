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

Feature: NewSession feature

    Scenario: asking server to create a new session
        Given machine manager server is up
        And cartesi machine default config description
        When client asks machine manager server to create a new session
        Then server returns correct machine hash

    Scenario: asking server to create an existing session without force
        Given machine manager server is up
        And cartesi machine default config description
        And some session exists
        When client asks machine manager server to create a new session with the same session id when forcing is disabled
        Then machine manager server returns an InvalidArgument error

    Scenario: asking server to create an existing session with force
        Given machine manager server is up
        And cartesi machine default config description
        And some session exists
        When client asks machine manager server to create a new session with the same session id when forcing is enabled
        Then server returns correct machine hash
