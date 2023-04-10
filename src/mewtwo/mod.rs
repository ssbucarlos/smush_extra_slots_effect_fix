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

#[acmd_script( agent = "mewtwo", script = "effect_downattacku", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_downattacku(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -0.5, 4.6, 5.0, 0.0, -60.0, -170.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 2.0, 3.3, -4.0, 0.0, 110.0, -195.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, true);
    }
}

#[acmd_script( agent = "mewtwo", script = "effect_downattackd", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_downattackd(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -0.5, 4.6, 5.0, 0.0, -60.0, -170.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 2.0, 3.3, -4.0, 0.0, 110.0, -195.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, true);
    }
}

#[acmd_script( agent = "mewtwo", script = "effect_attackairhi", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackairhi(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0.0, 9.5, -4.0, 0.0, 30.0, 90.0, 1.15, true, *EF_FLIP_YZ, slot_wrapped);
    }
}

#[acmd_script( agent = "mewtwo", script = "effect_attackairb", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackairb(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0.0, 13.5, -6.0, 180.0, 45.0, 90.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.05);
    }
}

#[acmd_script( agent = "mewtwo", script = "effect_attackhi3", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackhi3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 0.0, 14.0, 1.5, 0.0, 10.0, 90.0, 1.05, true, *EF_FLIP_YZ, slot_wrapped);
    }
}

#[acmd_script( agent = "mewtwo", script = "effect_attacks3lw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacks3lw(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 3.0, 3.5, 10.0, 0.0, -85.0, -13.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "mewtwo", script = "effect_attacks3hi", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacks3hi(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 3.0, 10.0, 10.0, 0.0, -85.0, 15.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "mewtwo", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacklw3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -5.5, 5.0, 4.5, 0.0, -70.0, 190.0, 1.05, true, *EF_FLIP_YZ, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
}

#[acmd_script( agent = "mewtwo", script = "effect_attacks3", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacks3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 3.0, 7.0, 10.0, 0.0, -85.0, -15.0, 1.1, true, *EF_FLIP_YZ, slot_wrapped);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "mewtwo", script = "effect_cliffattack", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_cliffattack(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 23, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), -3.0, 5.7, 4.5, 190.0, -80.0, 0.0, 0.95, true, *EF_FLIP_YZ, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "mewtwo", script = "effect_throwlw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_throwlw(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_FOLLOW_FLIP_arg13(fighter, Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("mewtwo_tail_attack_a_01"), Hash40::new("top"), 1.0, 12.0, 4.0, 180.0, -130.0, 85.0, 1.0, true, *EF_FLIP_YZ, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_tail_attack_a_01"), false, false);
    }
}



pub fn install() {
    smashline::install_acmd_scripts!(
        effect_downattacku,
        effect_downattackd,
        effect_attackairhi,
        effect_attackairb,
        effect_attackhi3,
        effect_attacks3lw,
        effect_attacks3hi,
        effect_attacklw3,
        effect_attacks3,
        effect_cliffattack,
        effect_throwlw,
    );
}