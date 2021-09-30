use crate::basic_types::{ComparativeType, KeyWordType, OpertaionType, TokenType};

#[derive(Clone, Copy)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<i64>,
    pub defered: bool,
    pub operation_type: Option<OpertaionType>,
    pub keyword_type: Option<KeyWordType>,
    pub comparative_type: Option<ComparativeType>,
    pub references: Option<i64>,
    pub otherwise: Option<i64>,
    pub id: Option<i64>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        operation_type: Option<OpertaionType>,
        keyword_type: Option<KeyWordType>,
        comparative_type: Option<ComparativeType>,
        value: Option<i64>,
        defered: bool,
        id: Option<i64>,
        references: Option<i64>,
        otherwise: Option<i64>,
    ) -> Token {
        Token {
            token_type,
            operation_type,
            keyword_type,
            comparative_type,
            value,
            defered,
            references,
            id,
            otherwise
        }
    }

    pub fn change_defered(&mut self, defer: bool) {
        self.defered = defer;
    }

    pub fn change_reference(&mut self, references: Option<i64>) {
        self.references = references;
    }

    pub fn change_otherwise(&mut self, otherwise: Option<i64>) {
        self.otherwise = otherwise;
    }

    pub fn get_keyword_type(self) -> Option<KeyWordType> {
        self.keyword_type
    }

    pub fn get_comparative_type(self) -> Option<ComparativeType> {
        self.comparative_type
    }

    pub fn get_operation_type(self) -> Option<OpertaionType> {
        self.operation_type
    }

    pub fn get_id(self) -> Option<i64> {
        self.id
    }

    fn op_no_args_operation(operation_type: OpertaionType) -> Token {
        Token::new(
            TokenType::Opertaion,
            Some(operation_type),
            None,
            None,
            None,
            false,
            None,
            None,
            None,
        )
    }
    fn cmp_no_args_operation(cmp_type: ComparativeType) -> Token {
        Token::new(
            TokenType::Comparative,
            None,
            None,
            Some(cmp_type),     
            None,
            false,
            None,
            None,
            None,
        )
    }
}

// opertaions with args
impl Token {
    pub fn op_push(x: i64) -> Token {
        Token::new(
            TokenType::Opertaion,
            Some(OpertaionType::Push),
            None,
            None,
            Some(x),
            false,
            None,
            None,
            None,
        )
    }
}

// keywords
impl Token {
    pub fn kw_while(id: i64) -> Token {
        Token::new(
            TokenType::KeyWord,
            None,
            Some(KeyWordType::While),
            None,
            None,
            false,
            Some(id),
            Some(id),
            None,
        )
    }

    pub fn kw_end(id: i64, references: i64) -> Token {
        Token::new(
            TokenType::KeyWord,
            None,
            Some(KeyWordType::End),
            None,
            None,
            false,
            Some(references),
            Some(id),
            None,
        )
    }

    pub fn kw_do(id: i64, references: i64) -> Token {
        Token::new(
            TokenType::KeyWord,
            None,
            Some(KeyWordType::Do),
            None,
            None,
            false,
            Some(references),
            Some(id),
            Some(references),
        )
    }
    pub fn kw_else(id: i64, references: i64) -> Token {
        Token::new(
            TokenType::KeyWord,
            None,
            Some(KeyWordType::Else),
            None,
            None,
            false,
            Some(references),
            Some(id),
            None,
        )
    }
    
}


// no args operations
impl Token {
    pub fn op_dump() -> Token {
        Token::op_no_args_operation(OpertaionType::Dump)
    }

    pub fn op_plus() -> Token {
        Token::op_no_args_operation(OpertaionType::Plus)
    }

    pub fn op_minus() -> Token {
        Token::op_no_args_operation(OpertaionType::Minus)
    }

    pub fn op_mul() -> Token {
        Token::op_no_args_operation(OpertaionType::Mul)
    }

    pub fn op_div() -> Token {
        Token::op_no_args_operation(OpertaionType::Div)
    }

    pub fn op_mod() -> Token {
        Token::op_no_args_operation(OpertaionType::Mod)
    }

    pub fn op_shr() -> Token {
        Token::op_no_args_operation(OpertaionType::Shr)
    }

    pub fn op_shl() -> Token {
        Token::op_no_args_operation(OpertaionType::Shl)
    }

    pub fn op_space() -> Token {
        Token::op_no_args_operation(OpertaionType::PutSpace)
    }

    pub fn op_band() -> Token {
        Token::op_no_args_operation(OpertaionType::Band)
    }

    pub fn op_bor() -> Token {
        Token::op_no_args_operation(OpertaionType::Bor)
    }

    pub fn op_xor() -> Token {
        Token::op_no_args_operation(OpertaionType::Xor)
    }

    pub fn op_blank() -> Token {
        Token::op_no_args_operation(OpertaionType::Blank)
    }


    pub fn op_dup() -> Token {
        Token::op_no_args_operation(OpertaionType::Dup)
    }

    pub fn op_dup2() -> Token {
        Token::op_no_args_operation(OpertaionType::Dup2)
    }

    pub fn op_swap() -> Token {
        Token::op_no_args_operation(OpertaionType::Swap)
    }

    pub fn op_pop() -> Token {
        Token::op_no_args_operation(OpertaionType::Pop)
    }

    pub fn op_put() -> Token {
        Token::op_no_args_operation(OpertaionType::Put)
    }

    pub fn op_swap2() -> Token {
        Token::op_no_args_operation(OpertaionType::Swap2)
    }

    pub fn cmp_less() -> Token {
        Token::cmp_no_args_operation(ComparativeType::Less)
    }

    pub fn cmp_less_eq() -> Token {
        Token::cmp_no_args_operation(ComparativeType::LessEq)
    }

    pub fn cmp_greater() -> Token {
        Token::cmp_no_args_operation(ComparativeType::Greater)
    }

    pub fn cmp_greater_eq() -> Token {
        Token::cmp_no_args_operation(ComparativeType::GreaterEq)
    }

    pub fn cmp_equal() -> Token {
        Token::cmp_no_args_operation(ComparativeType::Equal)
    }

    pub fn cmp_and() -> Token {
        Token::cmp_no_args_operation(ComparativeType::And)
    }

    pub fn cmp_or() -> Token {
        Token::cmp_no_args_operation(ComparativeType::Or)
    }

    pub fn cmp_not() -> Token {
        Token::cmp_no_args_operation(ComparativeType::Not)
    }



}
