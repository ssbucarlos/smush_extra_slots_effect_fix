use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};
use crate::helper;

#[acmd_script( agent = "yoshi", script = "effect_entryr", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_entryr(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("yoshi_entry_01"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
        macros::EFFECT(fighter, Hash40::new("yoshi_tamagoumi_smoke"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 49.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 71.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 2.3, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_ZX);
    }
}

#[acmd_script( agent = "yoshi", script = "effect_entryl", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_entryl(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("yoshi_entry_01"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
        macros::EFFECT(fighter, Hash40::new("yoshi_tamagoumi_smoke"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 49.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 71.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 2.3, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_ZX);
    }
}

#[acmd_script( agent = "yoshi", script = "effect_specialsloop", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialsloop(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_METAL) {
        if macros::is_excute(fighter){
            macros::EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_metal"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
        } 
    }
    else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GOLD) {
        if macros::is_excute(fighter){
            macros::EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_gold"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPYCLOAK) {
        if macros::is_excute(fighter){
            macros::EFFECT_FOLLOW(fighter, Hash40::new("null"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    else if helper::FT_IS_SAME_FIGHTER_CATEGORY(fighter, *METAMON) {
        if macros::is_excute(fighter){
            macros::EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_metamon"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    else if helper::FT_IS_SAME_FIGHTER_CATEGORY(fighter, *LIGHT) {
        if macros::is_excute(fighter){
            macros::EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_light"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    else if helper::FT_IS_SAME_FIGHTER_CATEGORY(fighter, *DARK) { 
        if macros::is_excute(fighter){
            macros::EFFECT_FOLLOW(fighter, Hash40::new("yoshi_gorogorotamago_dark"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    else {
        if macros::is_excute(fighter){
            macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("yoshi_gorogorotamago_01"), Hash40::new("egg"), 0, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
        }
    }
    
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    wait(fighter.lua_state_agent, 5.0);
}


#[acmd_script( agent = "yoshi", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackairlw(agent: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    let effect_name = std::format!("yoshi_air_trace_{:02}", slot_wrapped+1);

    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new(&effect_name), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
    }

    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new(&effect_name), true, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_entryr,
        effect_entryl,
        effect_specialsloop,
        effect_attackairlw,
    );
}