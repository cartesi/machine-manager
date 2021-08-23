Feature: SessionRun feature

    Background:
        Given a pristine machine manager server session

    Scenario: SessionRun run from pristine machine
        Given the cycles array 0,15,30,45,60 to run the machine
        When client asks server to run session
        Then server returns machine hashes:
            | cycles | hash                                                             |
            |   0    | 8009A430CFFD6013F9224B935DD0C9426200C59E3885DF2E46DBCD7975C61305 |
            |   15   | F081DAF8658D8D346F71FC9DFE14622767CD9FD470F7081E48A300B1455B2321 |
            |   30   | D3A20ABDB8E5CF05D4924597FE45D6644090BCC2BBBD9F21F9860E39DF6824AF |
            |   45   | 6420559E65D6708DB991C057087BAE6A20022956F688779868F86E2A190A05CB |
            |   60   | 1DEC67CBB473B140979EB892A86412CD8FEBC5F6394EDE2BC2D0728EC0FBC73B |

    Scenario: SessionRun run with rollback forcing

        # If first final_cycle < machine cycle and first final_cycle > snapshot cycle,
        # machine manager server forces cartesi machine rollback

        Given the machine executed with cycles 0,30,60
        And the cycles array 15,30,45 to run the machine
        When client asks server to run session
        Then server returns machine hashes:
            | cycles | hash                                                             |
            |   15   | F081DAF8658D8D346F71FC9DFE14622767CD9FD470F7081E48A300B1455B2321 |
            |   30   | D3A20ABDB8E5CF05D4924597FE45D6644090BCC2BBBD9F21F9860E39DF6824AF |
            |   45   | 6420559E65D6708DB991C057087BAE6A20022956F688779868F86E2A190A05CB |

    Scenario: SessionRun run with recreation forcing

        # If first final cycle < machine cycle and first final cycle < snapshot cycle,
        # machine manager server forces cartesi machine recreation.

        Given the machine executed with cycles 0,30,60
        And the cycles array 1,5,10 to run the machine
        When client asks server to run session
        Then server returns machine hashes:
            | cycles | hash                                                             |
            |   1    | 4E8BC27937011A2586185D8A7E53ED9B5ADC533D595DD35AADFA2FC0D4E1B54D |
            |   5    | C9BAF157B0F01B21A688E0B60C3128F77FD664E8004779C568843AF4F682F8F1 |
            |   10   | 33B32BDBC3BE8C70454C4F4F48994E5D3236DB61D2278320AA64201C87F03DDE |

    Scenario: SessionRun run with no need for any special effort

        # If first final cycle > machine cycle no special effort should be needed.

        Given the machine executed with cycles 0,30,60
        And the cycles array 15 to run the machine
        When client asks server to run session
        Then server returns machine hashes:
            | cycles | hash                                                             |
            |   15   | F081DAF8658D8D346F71FC9DFE14622767CD9FD470F7081E48A300B1455B2321 |

    Scenario: SessionRun long run
        Given the cycles array 500000000 to run the machine
        When client asks server to run session
        Then server returns machine hashes:
            |  cycles   | hash                                                             |
            | 500000000 | 98B7AA1154DB89B083725F2DA7782F1C8432392E2EC34426EEE23700A1AE4521 |
