Feature: SessionGetProof feature

    Scenario Outline: asking for proofs with different parameters
        Given machine manager server is up
        And a machine manager server with a machine executed for <cycle> final cycles
        When the machine manager server asks machine for proof on cycle <cycle> for address <address> with log2_size <size>
        Then server returns proof which SHA256 sum is <hash>

        Examples:
            | cycle |        address      | size |                             hash                                 |
            |  30   |          288        |  3   | 5CBD4721528040BDFD9D292626647AA8C123CB8937F9ED9498B61AFBBF8AA851 |
            |  30   |          288        |  4   | BFC8C9F0602C6CAAE87CCE97C128BC2E00A3B788680FA9C6A1CEC93447A7BDAF |
