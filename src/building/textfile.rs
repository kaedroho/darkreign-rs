use textfile::{Token, lex};
use building::BuildingType;


fn walk_parameter_list<I: Iterator<Item=Token>>(tokens_iter: &mut I) -> Result<Vec<String>, String> {
    if tokens_iter.next() != Some(Token::OpenRoundBracket) {
        return Err("unexpected token".to_string());
    }

    let mut parameter_list = Vec::new();
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Whitespace => {},
            Token::Word(word) => parameter_list.push(word),
            Token::CloseRoundBracket => return Ok(parameter_list),
            _ => return Err("unexpected token".to_string()),
        }
    }

    Err("unexpected end of input".to_string())
}


fn walk_type_definition<I: Iterator<Item=Token>>(tokens_iter: &mut I) -> Result<BuildingType, String> {
    let mut parameter_list = try!(walk_parameter_list(tokens_iter));

    if parameter_list.len() != 1 {
        return Err("unexpected number of parameters (expected 1)".to_string());
    }

    let mut building_type = BuildingType::new(parameter_list.pop().unwrap());

    if tokens_iter.next() != Some(Token::OpenCurlyBracket) {
        return Err("unexpected token".to_string());
    }

    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Whitespace | Token::Comment => {},
            Token::Word(word) => {
                match word.as_ref() {
                    "SetBuildingImages" => {
                        println!("IMAGES {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetDescription" => {
                        println!("DESCRIPTION {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetRequirements" => {
                        tokens_iter.next(); // whitespace

                        if tokens_iter.next() != Some(Token::OpenCurlyBracket) {
                            return Err("unexpected token".to_string());
                        }

                        while let Some(token) = tokens_iter.next() {
                            match token {
                                Token::Whitespace | Token::Comment => {},
                                Token::Word(word) => {
                                    match word.as_ref() {
                                        "SetType" => {
                                            println!("REQUIREMENTS TYPE {:?}", walk_parameter_list(tokens_iter));
                                        }
                                        "SetPrereqs" => {
                                            println!("REQUIREMENTS PREREQS {:?}", walk_parameter_list(tokens_iter));
                                        }
                                        "SetMaker" => {
                                            println!("REQUIREMENTS MAKER {:?}", walk_parameter_list(tokens_iter));
                                        }
                                        "SetEquivalence" => {
                                            println!("REQUIREMENTS EQUIVALENCE {:?}", walk_parameter_list(tokens_iter));
                                        }
                                        "SetTechLevel" => {
                                            println!("REQUIREMENTS TECH LEVEL {:?}", walk_parameter_list(tokens_iter));
                                        }
                                        _ => panic!("unexpected requirements {}", word),
                                    }
                                }
                                Token::CloseCurlyBracket => break,
                                _ => panic!("unexpected token {:?}", token),
                            }
                        }
                    }
                    "SetEfficiencyResource" => {
                        let mut open_curly_bracket_token = tokens_iter.next();
                        while open_curly_bracket_token != Some(Token::OpenCurlyBracket) {
                            open_curly_bracket_token = tokens_iter.next();
                        }

                        // FIXME: Currently just skips to the next close bracket
                        while let Some(token) = tokens_iter.next() {
                            match token {
                                Token::CloseCurlyBracket => break,
                                _ => {},
                            }
                        }
                    }
                    "SetSpyType" => {
                        println!("SPY TYPE {:?}", walk_parameter_list(tokens_iter));
                    }
                    "NeedResource" => {
                        println!("NEED RESOURCE {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetSide" => {
                        println!("SIDE {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetBay" => {
                        println!("SIDE {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetRepairCost" => {
                        println!("REPAIR COST {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetHitpoints" => {
                        println!("HITPOINTS {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetSeeingRange" => {
                        println!("SEEING RANGE {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetSeeingHeight" => {
                        println!("SEEING HEIGHT {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetCost" => {
                        println!("COST {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetSell" => {
                        println!("SELL {:?}", walk_parameter_list(tokens_iter));
                    }
                    "CanMake" => {
                        println!("CAN MAKE {:?}", walk_parameter_list(tokens_iter));
                    }
                    "MakesCrater" => {
                        println!("MAKES CRATER {:?}", walk_parameter_list(tokens_iter));
                    }
                    "GivesMiniMap" => {
                        println!("GIVES MINIMAP {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetShadowImage" => {
                        println!("SET SHADOW IMAGE {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetTransportUnit" => {
                        println!("SET TRANSPORT UNIT {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetVulnerability" => {
                        println!("SET VULNERABILITY {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetHealthExplosion" => {
                        println!("SET HEALTH EXPLOSION {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetRepairActionIndicator" => {
                        println!("SET REPAIR ACTION INDICATOR {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetDeathSfx" => {
                        println!("SET DEATH SFX {:?}", walk_parameter_list(tokens_iter));
                    }
                    "IsUpgradeOf" => {
                        println!("IS UPGRADE OF {:?}", walk_parameter_list(tokens_iter));
                    }
                    "IsDecoyOf" => {
                        println!("IS DECOY OF {:?}", walk_parameter_list(tokens_iter));
                    }
                    "IsNotBuilderEater" => {
                        println!("IS NOT BUILDER EATER {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetResource" => {
                        println!("RESOURCE {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetResourceSale" => {
                        println!("RESOURCE SALE {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetResourceSaleAnimation" => {
                        println!("RESOURCE SALE ANIMATION {:?}", walk_parameter_list(tokens_iter));
                    }
                    "AssociatedUnit" => {
                        println!("ASSOCIATED UNIT {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SupplyResource" => {
                        println!("SUPPLY RESOURCE {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetIdleAnimation" => {
                        println!("IDLE ANIMATION {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetEquivalentBuilding" => {
                        println!("EQUIVALENT BUILDING {:?}", walk_parameter_list(tokens_iter));
                    }
                    "ActivePart" => {
                        println!("ACTIVE PART {:?}", walk_parameter_list(tokens_iter));
                    }
                    "IsBuiltFromEdge" => {
                        println!("IS BUILT FROM EDGE {:?}", walk_parameter_list(tokens_iter));
                    }
                    "CanHeal" => {
                        println!("CAN HEAL {:?}", walk_parameter_list(tokens_iter));
                    }
                    "CanRepair" => {
                        println!("CAN REPAIR {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetRepairAnimation" => {
                        println!("REPAIR ANIMATION {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetRooms" => {
                        println!("ROOMS {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetBoardAnimation" => {
                        println!("BOARD ANIMATION {:?}", walk_parameter_list(tokens_iter));
                    }
                    "CanRearmFlyer" => {
                        println!("CAN REARM FLYER {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetRearmAnimation" => {
                        println!("REARM ANIMATION {:?}", walk_parameter_list(tokens_iter));
                    }
                    "IsBridge" => {
                        println!("IS BRIDGE {:?}", walk_parameter_list(tokens_iter));
                    }
                    "IsTeleport" => {
                        println!("IS TELEPORT {:?}", walk_parameter_list(tokens_iter));
                    }
                    "SetCPUGainPercent" => {
                        println!("CPU GAIN PERCENT {:?}", walk_parameter_list(tokens_iter));
                    }
                    _ => panic!("unexpected statement {}", word),
                }
            }
            Token::CloseCurlyBracket => return Ok(building_type),
            _ => panic!("unexpected token {:?}", token),
        }
    }

    Err("Foo".to_string())
}


pub fn parse_build_textfile(data: &[u8]) -> Result<Vec<BuildingType>, String> {
    let tokens = try!(lex(data));
    let mut tokens_iter = tokens.into_iter();

    let mut building_types = Vec::new();
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Whitespace | Token::Comment => {},
            Token::Word(word) => {
                if &word == "DefineBuildingType" {
                    building_types.push(try!(walk_type_definition(&mut tokens_iter)));
                } else {
                    panic!("unexpected word token {:?}", word);
                }
            }
            _ => panic!("unexpected token {:?}", token),
        }
    }

    Ok(building_types)
}
