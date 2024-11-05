use super::sword::{Stats, Entity};


#[derive(Clone)]
pub struct Wizard {
    hit_points: u32, 
    mana: u32,
    spells: Vec<Spell>
}


impl Wizard {
    pub fn new(hit_points: u32, mana: u32) -> Self {
        Self {
            hit_points, 
            mana,
            spells: Spell::create_basic_spells()
        }
    }

    pub fn win_with_least_mana(&self, other: &Entity) -> u32 {
        let mut least_mana = u32::MAX;

        //Spells will be independent of wizard as they need the wizard to do operations on 
        
        let mut wizard = self.clone(); 
        wizard.spells.sort_by(|a, b| a.get_mana_cost().cmp(&b.get_mana_cost()));
        
        let spell_len = self.spells.len();
        for spell_idx in 0..spell_len {
            let mut wizard_clone = wizard.clone();
            let mut opp_clone = other.clone();

            let spell_type = wizard_clone.spells[spell_idx].get_spell_type();

            wizard_clone.cast(&spell_type,&mut opp_clone, false); 

            if let Some(mana) = Self::fight_recursively(wizard_clone, opp_clone) {
                least_mana = least_mana.min(mana);
            }
        }
        least_mana
    }

    fn check_status_effects(&self) -> bool {
        for spell in self.spells.iter() {
            if spell.is_active() { return true } 
        }
        false
    }

    fn fight_recursively(mut wiz: Wizard, mut opp: Entity) -> Option<u32> {
        if wiz.hit_points == 0 {
            return None
        } else if opp.get_hit_points() == 0 {
            return Some(wiz.mana)
        }
        let mut least: Option<u32> = None;

        
        let spell_len = wiz.spells.len();
        for spell_idx in 0..spell_len {
            let spell_type = wiz.spells[spell_idx].get_spell_type();

            wiz.cast(&spell_type,&mut opp, false); 
        }

        if wiz.check_status_effects() {
            if let Some(val) = Self::fight_recursively(wiz.clone(), opp.clone()) {
                least = least
                .map(|current_least| val.min(current_least))
                .or(Some(val));
            }
        }

        //todo Let opponent do dmg
        wiz.take_damage(opp.get_dmg());
        
        let len = wiz.spells.len();
        for spell_idx in 0..len {
            let mut new_wiz = wiz.clone(); 
            let spell_type = new_wiz.spells[spell_idx].get_spell_type();
            if !new_wiz.can_cast(&spell_type) { continue }

            new_wiz.cast(&spell_type, &mut opp, true);

            Self::fight_recursively(new_wiz, opp.clone());
        }
        None
    }

    fn take_damage(&mut self, mut dmg: u32) {
        let spell = self.spells.iter().find(|spell| spell.get_spell_type() == SpellType::Shield).unwrap();

        if spell.is_active() { dmg = 1.max(dmg as i32 - 7) as u32; }

        self.hit_points = 0.max(self.hit_points as i32 - dmg as i32) as u32;
    }

    
}

impl Wizard {
    fn can_cast(&self, spell_type: &SpellType) -> bool {
        match spell_type {
            SpellType::MagicMissile => self.mana >= 53,
            SpellType::Drain => self.mana >= 73,
            SpellType::Shield => self.mana >= 113,
            SpellType::Poison => self.mana >= 173,
            SpellType::Recharge => self.mana >= 229,
        }
    }

    pub fn cast(&mut self, spell_type: &SpellType, enemy: &mut impl Stats, cast: bool) {
        match spell_type {
            SpellType::MagicMissile => self.cast_magic_missile(enemy, cast),
            SpellType::Drain => self.cast_drain(enemy, cast),
            SpellType::Shield => self.cast_shield(enemy, cast),
            SpellType::Poison => self.cast_poison(enemy, cast),
            SpellType::Recharge => self.cast_recharge(enemy, cast),
        }
    }

    fn cast_magic_missile(&mut self, enemy: &mut impl Stats, cast: bool) { 
        if !cast { return }
        let spell = self.spells.iter_mut().find(|spell| spell.get_spell_type() == SpellType::MagicMissile).unwrap();

        enemy.take_damage(spell.get_effect()); 
        self.decrease_mana(53);
    }

    fn cast_drain(&mut self, enemy: &mut impl Stats, cast: bool) {
        if !cast { return }

        let spell = self.spells.iter_mut().find(|spell| spell.get_spell_type() == SpellType::Drain).unwrap();
        
        enemy.take_damage(spell.get_effect());
        self.hit_points += spell.get_effect(); 
        self.decrease_mana(73);
    }

    fn cast_shield(&mut self, _enemy: &mut impl Stats, cast: bool) {
        let spell = self.spells.iter_mut().find(|spell| spell.get_spell_type() == SpellType::Shield).unwrap();
        if cast && !spell.is_active() { 
            spell.activate(); 
            return self.decrease_mana(113)
        }

        spell.decrease_status();
    }

    fn cast_poison(&mut self, enemy: &mut impl Stats, cast: bool) {
        let spell = self.spells.iter_mut().find(|spell| spell.get_spell_type() == SpellType::Poison).unwrap();
        if cast && !spell.is_active() { 
            spell.activate(); 
            return self.decrease_mana(173)
        }

        spell.decrease_status();
        enemy.take_damage(3);
    }

    fn cast_recharge(&mut self, _enemy: &mut impl Stats, cast: bool) {
        let spell = self.spells.iter_mut().find(|spell| spell.get_spell_type() == SpellType::Poison).unwrap();
        if cast && !spell.is_active() { 
            spell.activate(); 
            return self.decrease_mana(229)
        }

        spell.decrease_status();
        self.mana += 101; 
    }

    fn decrease_mana(&mut self, mana_cost: u32) { self.mana = 0.max(self.mana as i32 - mana_cost as i32) as u32 }

}


#[derive(Clone)]
pub struct Spell {
    spell_type: SpellType,
    mana_cost: u32, 
    status_active: u32,
    status_max_time: u32,
    effect: u32
}

#[derive(Clone, PartialEq, Debug)]
pub enum SpellType {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge
}

impl Spell {
    pub fn new(spell_type: SpellType, mana_cost: u32, active: u32, duration: u32, effect: u32) -> Self {
        Self { spell_type, mana_cost, status_active: active, status_max_time: duration, effect }
    }
    pub fn is_active(&self) -> bool { self.status_active != 0 }
    
    
    pub fn get_spell_type(&self) -> SpellType { self.spell_type.clone() }
    pub fn get_mana_cost(&self) -> u32 { self.mana_cost }
    pub fn get_effect(&self) -> u32 { self.effect }

    pub fn activate(&mut self) { self.status_active = self.status_max_time }

    pub fn decrease_status(&mut self) {
        if self.is_active() { self.status_active -= 1 }
    }

    pub fn create_basic_spells() -> Vec<Spell> {
        vec![
            Spell::new(SpellType::MagicMissile, 53, 0, 1, 4),          // Does 4 damage instantly
            Spell::new(SpellType::Drain, 73, 0, 1, 2),                 // Deals 2 damage, heals 2 HP
            Spell::new(SpellType::Shield, 113, 1, 6, 7),              // Increases armor for 6 turns
            Spell::new(SpellType::Poison, 173, 1, 6, 3),              // Deals 3 damage each turn for 6 turns
            Spell::new(SpellType::Recharge, 229, 1, 5, 101),          // Gives 101 mana each turn for 5 turns
        ]
    }
}




#[cfg(test)]
mod tests {
    use crate::rpg_simulator::entities::sword::Entity;

    use super::*;
    #[test] 
    fn test_wizard_battle_logic() {
        let opp = Entity::new(13, 8, 0);
        let wizard = Wizard::new(10, 250);

        assert_eq!(236, wizard.win_with_least_mana(&opp));
    }

    #[test]
    fn test_magic_missile() {
        let mut spell = Spell::new(SpellType::MagicMissile, 53, 0, 0, 4);

        spell.activate();
        assert_eq!(spell.get_spell_type(), SpellType::MagicMissile);
        assert_eq!(spell.is_active(), false);

        spell.activate();
        assert_eq!(spell.is_active(), false);

        assert_eq!(spell.effect, 4);
        assert_eq!(53, spell.mana_cost);
    }

}
