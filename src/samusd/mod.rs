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

#[acmd_script( agent = "samusd", script = "effect_aircatchhit", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_aircatchhit(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.3, true, slot_wrapped);
    }
}

#[acmd_script( agent = "samusd", script = "effect_aircatchpose", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_aircatchpose(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1.3, true, slot_wrapped);
    }
}

#[acmd_script( agent = "samusd", script = "effect_catchturn", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_catchturn(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1.3, true, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_gbeam_vanish"), Hash40::new("armr"), 7, 0, -0.5, 0, 0, 0, 0.4, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_catch", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_catch(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1.3, true, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_gbeam_vanish"), Hash40::new("armr"), 7, 0, -0.5, 0, 0, 0, 0.4, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_finalair", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_finalair(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            helper::EFFECT_GLOBAL(fighter, Hash40::new("samusd_final_stone"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, false);
        }
    }
    else{
        if macros::is_excute(fighter) {
            helper::EFFECT_GLOBAL(fighter, Hash40::new("samusd_final_stone"), 0.0, 0.0, 0.0, 0.0, 180.0, 0.0, 1.0, false);
        }
    }
}

#[acmd_script( agent = "samusd", script = "effect_final", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_final(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            helper::EFFECT_GLOBAL(fighter, Hash40::new("samusd_final_stone"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, false);
        }
    }
    else{
        if macros::is_excute(fighter) {
            helper::EFFECT_GLOBAL(fighter, Hash40::new("samusd_final_stone"), 0.0, 0.0, 0.0, 0.0, 180.0, 0.0, 1.0, false);
        }
    }
}

#[acmd_script( agent = "samusd", script = "effect_finalstart", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_finalstart(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
        EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_samusd_final"), false, false, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_final_start"), Hash40::new("top"), 0, 12.5, 19, 0, 0, 0, 1, false);
    }
}

#[acmd_script( agent = "samusd", script = "effect_aircatch", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_aircatch(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1.3, true, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_gbeam_shot"), Hash40::new("armr"), 7, 0, -0.5, 0, 0, 0, 0.75, true);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_gbeam_vanish"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 0.7, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_specialair", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialair(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 1.3, true, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samusd_missile_shot"), Hash40::new("haver"), 2.2, 0.379, -0.15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_catchdash", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_catchdash(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1.3, true, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_gbeam_vanish"), Hash40::new("armr"), 7, 0, -0.5, 0, 0, 0, 0.4, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_special", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_special(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 1.3, true, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samusd_missile_shot"), Hash40::new("haver"), 2.2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_specialairs", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialairs(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 1.3, true, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samusd_missile_shot"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samusd_missile_straight_smoke"), Hash40::new("armr"), 3, -1, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_finalairend", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_finalairend(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_final_aura"), false, true);
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
        EffectModule::remove_screen(fighter.module_accessor, Hash40::new("bg_samusd_final"), -1);
    }
    frame(fighter.lua_state_agent, 2.0);
    for _ in 0..11 {
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samusd_final_after"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 360, 360, 360, true);
        macros::EFFECT(fighter, Hash40::new("samusd_final_flash"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 360, 360, 360, true);
    }
    }
    frame(fighter.lua_state_agent, 69.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new_raw(0x1b905a71d7), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_finalend", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_finalend(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_final_aura"), false, true);
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
        EffectModule::remove_screen(fighter.module_accessor, Hash40::new("bg_samusd_final"), -1);
    }
    frame(fighter.lua_state_agent, 2.0);
    for _ in 0..11 {
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samusd_final_after"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 360, 360, 360, true);
        macros::EFFECT(fighter, Hash40::new("samusd_final_flash"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 360, 360, 360, true);
    }
    }
    frame(fighter.lua_state_agent, 69.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new_raw(0x1b905a71d7), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_attacks4lw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacks4lw(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 1.3, true, slot_wrapped);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, 3, 28, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_attacks4", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacks4(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 1.3, true, slot_wrapped);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, 3, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_specials", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specials(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 1.3, true, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samusd_missile_shot"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samusd_missile_straight_smoke"), Hash40::new("armr"), 3, -1, -1, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_throwhi", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_throwhi(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FLW_POS(fighter, Hash40::new("samusd_throw_hi"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_gbeam_vanish"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 0.6, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_attacks4hi", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacks4hi(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 1.3, true, slot_wrapped);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 10, 3, -32, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_throwlw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_throwlw(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 1.0);
    for _ in 0..4 {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 10, 0, 0, 0, 0, 0, 1.5, 4, 4, 4, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 4.0);
}
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 15, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 4, 4, 4, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 49.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_gbeam_vanish"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 0.6, true);
    }
    frame(fighter.lua_state_agent, 52.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_throwf", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_throwf(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 1.0);
    for _ in 0..3 {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 4, 4, 4, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 4.0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 4, 4, 4, 0, 0, 0, false);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_gbeam_vanish"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 0.6, true);
    }
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
    }
}

#[acmd_script( agent = "samusd", script = "effect_throwb", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_throwb(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 4, 4, 4, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 4, 4, 4, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 4.0);
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), -3, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_gbeam_vanish"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 0.6, true);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_aircatchhit,
        effect_aircatchpose,
        effect_catchturn,
        effect_catch,
        effect_finalair,
        effect_final,
        effect_finalstart,
        effect_aircatch,
        effect_specialair,
        effect_catchdash,
        effect_special,
        effect_specialairs,
        effect_finalairend,
        effect_finalend,
        effect_attacks4lw,
        effect_attacks4,
        effect_specials,
        effect_throwhi,
        effect_attacks4hi,
        effect_throwlw,
        effect_throwf,
        effect_throwb,
    );
}