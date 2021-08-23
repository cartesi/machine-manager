Feature: SessionGetProof feature

    Scenario Outline: SessionGetProof asking for proofs with different parameters
        Given a machine manager server with a machine executed for <cycle> final cycles
        When the machine manager server asks machine for proof on cycle <cycle> for address <address> with log2_size <size>
        Then server returns proof which SHA256 sum is <hash>

        Examples:
            | cycle |        address      | size |                             hash                                 |
            |  30   |          288        |  3   | 7874779554DC7FE0DF96D7571013905585813F5F1E98BBAF4B86FBCEDC122023 |
            |  30   |          288        |  4   | A110029E988EC98ABF663D2AA68B3FEC5CF0B0FA2D936BDC9326DE45FE976A92 |
            |   0   | 9223372036854775808 |  3   | 3A39238641C988DF4C062B4DA7AB7082B52EFF3FD03CB1B2A3B618D6203541BC |
