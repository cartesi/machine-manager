Feature: SessionStep feature

    Scenario Outline: steps with different cycles

        # - Step with initial cycle = 0 should happen on a new machine.
        # - Step with initial cycle < machine cycle and initial cycle > snapshot cycle should force machine rollback.
        # - Step with initial cycle < machine cycle and initial cycle < snapshot cycle should force machine recreation.
        # - Step with initial cycle > machine cycle should not require any special effort.
        # - Step with initial cycle = machine cycle, should not even require making a dummy run to get into machine
        #   cycle = initial cycle.

        Given machine manager server is up
        And a machine manager server with a machine executed for <cycle> final cycles
        When the machine manager server asks machine to step on initial cycle <cycle>
        Then server returns access log which SHA256 sum is <hash>

        Examples:
            | cycle |                             hash                                 |
            |   1   | 53E64F89AB04CDF1512E3110EEE7E8C25BB6CDD45961E80FCC6D3D557C5B6908 |
            |   21  | AB19FF37897D8931CE27CB04623EB1E63A7076104F9B7AC834617F614E4B53B0 |
            |   35  | 7CF3B9244A16F90EC3AD7C6D03F05307117E496EAFC9A6E0B0CD23B8F1FC9916 |
            |   30  | 89CFBE145339AAE5BD281FC4DC242500931690C2341B6E8D4B6A5B33AB9F2E32 |

    Scenario Outline: step on invalid cycle

        # For rust machine manager:
        # For operations ReadMemory/WriteMemory/Step/GetProof, in case where cycle argument is not equal to current
        # session cycle argument, return error. SessionRun request should be used to run machine to particular cycle.

        Given machine manager server is up
        And a machine manager server with a machine executed for <cycle> final cycles
        When the machine manager server asks machine to step on initial cycle <cycle>
        #Then machine manager server returns an Internal error
        Then server returns access log which SHA256 sum is <hash>

        Examples:
            | cycle |                             hash                                 |
            |   20  | 4E035EE7C71993627B3BEC9C294A5F546A56C7F5A6768C0E14503D7B33570218 |
