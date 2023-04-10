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

#[acmd_script( agent = "duckhunt", script = "effect_appealsl", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_appealsl(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 72.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 86.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_appealsr", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_appealsr(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 72.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 86.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_downboundu", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_downboundu(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        helper::DOWN_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -2, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -2, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_downboundd", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_downboundd(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        helper::DOWN_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -2, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -2, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_attackairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackairlw(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -4, 0, -55, 0, 270, 0.75, true, 0.9);
        }
    }
    else{
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, -4, 0, 45, 0, 270, 0.75, true, 0.9);
        }
    }
    
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, -4.3, 2, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 360, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckrot"), 0, 0, 0, 90, 0, 0, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 19.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 5.5, 4.5, 7.5, 114, 8.6, -17.7, 1, true);
        }
    }
    else{
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -5.8, 4.5, 7.5, 114, -7.8, 18, 1, true);
        }
    }
    frame(fighter.lua_state_agent, 20.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -1.35, -11, 0, 0, 0, 0, 1.2, true);
        }
    }
    else{
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 1.35, -11, 0, 0, 0, 0, 1.2, true);
        }
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc"), true, true);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
    helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckrot"), 0, 0, 0, 90, 0, 0, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_specialhi", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialhi(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather_long"), Hash40::new("duckrot"), 0, 4, 0, 20, 0, 0, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_jumpaerialback", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_jumpaerialback(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_appealhir", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_appealhir(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 93.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_magicpotairstartaerial", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_magicpotairstartaerial(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_jumpaerialfront", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_jumpaerialfront(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_magicpotairendaerial", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_magicpotairendaerial(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_appealhil", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_appealhil(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 93.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_fflowershootaerial", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_fflowershootaerial(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_genesisaeriallegs", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_genesisaeriallegs(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_win1", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_win1(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 55.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("duckhunt_win1_grass"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, 89, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    wait(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    wait(fighter.lua_state_agent, 25.0);
}

#[acmd_script( agent = "duckhunt", script = "effect_finalairend", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_finalairend(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_can"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit2"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit3"), true, true);
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather_long"), Hash40::new("duckrot"), 0, 4, 0, 20, 0, 0, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 72.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_finalend", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_finalend(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_can"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit2"), true, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_final_hit3"), true, true);
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather_long"), Hash40::new("duckrot"), 0, 4, 0, 20, 0, 0, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 72.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_attackhi3", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackhi3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 1, 8, -0.5, -90, 0, 0, 0.85, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacklw3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 0, 7, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 2, 1.5, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_attackairf", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackairf(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 7.5, 9, 0, 0, 0, 0.9, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 7.5, 3, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
        macros::LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), false, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_attack100end", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attack100end(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack100"), false, false);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 7, 5, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1, 1, 1);
        macros::LAST_EFFECT_SET_RATE(fighter, 2);
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("duckhunt_atk_smoke_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else{
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("duckhunt_atk_smoke_l"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
    }
    
    if macros::is_excute(fighter) {
        macros::EFFECT_FLIP(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 7, 18, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 360, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), 0, 0, 0, 0, 0, 0, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_attackairb", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackairb(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), -1.5, 6.8, -2, 0, 180, 0, 0.9, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), -1.5, 6.8, 2, 0, 180, 0, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
        macros::LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), false, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_attacks3hi", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacks3hi(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 9.5, 10, -20, 0, 0, 0.6, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 6.8, 1.2, -20, 0, 0, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), true, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), true, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_attacks3lw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacks3lw(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 5, 10, 25, 0, 0, 0.6, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 8, 3, 25, 0, 0, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), true, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), true, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

#[acmd_script( agent = "duckhunt", script = "effect_attacks3", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacks3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, slot_wrapped);
        helper::LAST_EFFECT_SET_WORK_INT(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("duckhunt_attack_line"), Hash40::new("duckhunt_attack_line"), Hash40::new("top"), 0, 7, 11, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line_b"), Hash40::new("sys_attack_line_b"), Hash40::new("top"), 0, 7, 1.7, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 1, 0.941, 0.392);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line_b"), true, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("duckhunt_attack_line"), true, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        helper::EFFECT_OFF_HANDLE(fighter, *FIGHTER_STATUS_WIN_WORK_INT_EFFECT_HANDLE);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_appealsl,
        effect_appealsr,
        effect_downboundu,
        effect_downboundd,
        effect_attackairlw,
        effect_specialhi,
        effect_jumpaerialback,
        effect_appealhir,
        effect_magicpotairstartaerial,
        effect_jumpaerialfront,
        effect_magicpotairendaerial,
        effect_appealhil,
        effect_fflowershootaerial,
        effect_genesisaeriallegs,
        effect_win1,
        effect_finalairend,
        effect_finalend,
        effect_attackhi3,
        effect_attacklw3,
        effect_attackairf,
        effect_attack100end,
        effect_attackairb,
        effect_attacks3hi,
        effect_attacks3lw,
        effect_attacks3,
    );
}