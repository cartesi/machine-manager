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
            |   1   | 94795FAC347B57CD41DBF3DF844451EA2329C25F1F354738D3C10B49F69FD409 |
            |   21  | 18AFC0146D1F903CB882A6DFEF8CD19CE63FEB32AE6DFFBD9EACFF6BD23EDCF4 |
            |   35  | 5A1873752302573D76B0918DA7F3387B4336E4FC29665E431501D9BD1A14D94B |
            |   30  | 7F16724BE2E18E8F280936EFC569B99DC883C4D8E2F9D3E5542D44EFEB2FE0BA |

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
            |   20  | 221C6956313B3F1B865354CC67CF4D3A36F4B020B23ECF1B02F8EC76F2D3D3B4 |
