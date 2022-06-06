use chrono::{DateTime, Utc};

pub struct Vet<'v> {
    id: String,
    vet_type: &'v VetType,
    contract: &'v VetTributes,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    name: String,
    email: String,
    class_register_id: String,
    vet_area: &'v VetArea,
    state: String,
    city: String,
    address: String,
    complement: String,
}

pub enum VetType {
    Public,
    Particular,
}

pub enum VetTributes {
    PF,
    PJ,
}

pub enum VetArea {
    General,
    Cancer,
    Surgery,
    Infections,
}

impl<'v> Vet<'v> {
    pub fn new(id: String, vet_type: &'v VetType,contract: &'v VetTributes, name: String,
               email: String, class_register_id: String, vet_area: &'v VetArea, state: String,
               city: String, address: String, complement: String) -> Vet<'v> {
        Vet {
            id,
            vet_type,
            contract,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name,
            email,
            class_register_id,
            vet_area,
            state,
            city,
            address,
            complement,
        }
    }

    pub fn get(&self) -> Vet<'v> {
        Vet {
            id: String::from(&self.id),
            vet_type: self.vet_type,
            contract: self.contract,
            created_at: self.created_at,
            updated_at: self.updated_at,
            name: String::from(&self.name),
            email: String::from(&self.email),
            class_register_id: String::from(&self.class_register_id),
            vet_area: self.vet_area,
            state: String::from(&self.state),
            city: String::from(&self.city),
            address: String::from(&self.address),
            complement: String::from(&self.complement),
        }
    }
}

