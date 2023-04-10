use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};
use crate::helper;

#[acmd_script( agent = "sonic", script = "effect_run", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_run(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_DISABLE_RUN_TRACE) {
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("sonic_runtrace"), Hash40::new("top"), 0, 0, 0, 180, 0, 0, -1, true, slot_wrapped);
                macros::LAST_EFFECT_SET_RATE(fighter, 1.09);
            }
            
        }
        else{
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("sonic_runtrace"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
                macros::LAST_EFFECT_SET_RATE(fighter, 1.09);
            }
        }
    }
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 4.0);
}

#[acmd_script( agent = "sonic", script = "effect_win2", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_win2(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("sonic_runtrace"), Hash40::new("throw"), 0, 0, 0, 180, 180, 0, -1, true, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.09);
        }
    }
    else{
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("sonic_runtrace"), Hash40::new("throw"), 0, 0, 0, 0, 180, 0, 1, true, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.09);
        }
    }
    if macros::is_excute(fighter) {
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    for _ in 0..16 {
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("throw"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("throw"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    wait(fighter.lua_state_agent, 2.0);
    }
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 65.0);
    if macros::is_excute(fighter) {
    macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("throw"), 4, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 70.0);
    if macros::is_excute(fighter) {
    macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("throw"), 7, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    macros::LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
    frame(fighter.lua_state_agent, 96.0);
    if macros::is_excute(fighter) {
    macros::EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 1.5, 0, 0, 0, 0, 0, 0.675, 0, 0, 0, 0, 0, 0, true);
    macros::LAST_EFFECT_SET_RATE(fighter, 0.78);
    }
    frame(fighter.lua_state_agent, 112.0);
    if macros::is_excute(fighter) {
    macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), -1, 1, 1, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}

#[acmd_script( agent = "sonic", script = "effect_appealsr", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_appealsr(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_DISABLE_RUN_TRACE) {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("sonic_appealruntrace"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, true, slot_wrapped);
            helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.76);
        }
    }
    frame(fighter.lua_state_agent, 70.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "sonic", script = "effect_appealsl", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_appealsl(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_DISABLE_RUN_TRACE) {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("sonic_appealruntrace"), Hash40::new("top"), 0, 0, 0, 180, 0, 0, -0.85, true, slot_wrapped);
            helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.76);
        }
    }
    frame(fighter.lua_state_agent, 70.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "sonic", script = "effect_win1", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_win1(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 1.0);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_DISABLE_RUN_TRACE) {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("sonic_runtrace"), Hash40::new("trans"), 0, 0, 0, 0, 90, 0, 1, true, slot_wrapped);
            helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    for _ in 0..8 {
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("trans"), 5, -0.5, 0, 0, 89, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("trans"), 5, -0.5, 0, 0, 89, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("trans"), 5, -0.5, 0, 0, 89, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("trans"), 5, -0.5, 0, 0, 89, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 1);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 48.0);
    for _ in 0..4 {
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("trans"), -5, -0.5, 0, 0, 90, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.9);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("trans"), -5, -0.5, 0, 0, 90, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.9);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("trans"), -5, -0.5, 0, 0, 90, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.9);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("trans"), -5, -0.5, 0, 0, 90, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.9);
        }
    }
    frame(fighter.lua_state_agent, 80.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 84.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_dash_smoke"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_turn_smoke"), true, true);
    }
    frame(fighter.lua_state_agent, 86.0);
    for _ in 0..5 {
        if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("trans"), 4, -0.5, 0, 0, -70, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 3.0);
    }
    frame(fighter.lua_state_agent, 102.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("trans"), 4, -0.5, 0, 0, -110, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 105.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("trans"), 0, -0.5, 0, 0, -110, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(
        effect_run,
        effect_win2,
        effect_appealsr,
        effect_appealsl,
    );
}
