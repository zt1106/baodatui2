use crate::card::PrimeOrSub;

pub enum LeadPattern {
    Valid(ValidLeadPattern),
    Invalid(InvalidLeadPattern),
}

pub enum ValidLeadPattern {
    SingleLead,
    MultiLead,
    SiblingLead(u32),
}

pub enum InvalidLeadPattern {
    WrongPatternLead(String),
    RuleAlreadyBrokenLead { pos: PrimeOrSub, width: u32 },
}
