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

#[acmd_script( agent = "packun", script = "effect_attackhi3", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackhi3(fighter: &mut L2CAgentBase) {
    let slot_wrapped = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 8;
    if slot_wrapped == 0 {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_01"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_01"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_01"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_01"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_01"), Hash40::new("top"), 0, 0, 0, 180, -110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_01"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_01"), Hash40::new("top"), 0, 0, 0, 0, 120, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_01"), false, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
        }
    }
    
    if slot_wrapped == 1 {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_02"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_02"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_02"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_02"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_02"), Hash40::new("top"), 0, 0, 0, 180, -110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_02"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_02"), Hash40::new("top"), 0, 0, 0, 0, 120, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_02"), false, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
        }
    }
    
    if slot_wrapped == 2 {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_03"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_03"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_03"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_03"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_03"), Hash40::new("top"), 0, 0, 0, 180, -110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_03"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_03"), Hash40::new("top"), 0, 0, 0, 0, 120, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_03"), false, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
        }
    }
    
    if slot_wrapped == 3 {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_04"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_04"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_04"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_04"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_04"), Hash40::new("top"), 0, 0, 0, 180, -110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_04"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_04"), Hash40::new("top"), 0, 0, 0, 0, 120, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_04"), false, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
        }
    }
    
    if slot_wrapped == 4 {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_05"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_05"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_05"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_05"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_05"), Hash40::new("top"), 0, 0, 0, 180, -110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_05"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_05"), Hash40::new("top"), 0, 0, 0, 0, 120, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_05"), false, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
        }
    }
    
    if slot_wrapped == 5 {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_06"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_06"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_06"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_06"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_06"), Hash40::new("top"), 0, 0, 0, 180, -110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_06"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_06"), Hash40::new("top"), 0, 0, 0, 0, 120, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_06"), false, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
        }
    }
    
    if slot_wrapped == 6 {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_07"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_07"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_07"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_07"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_07"), Hash40::new("top"), 0, 0, 0, 180, -110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_07"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_07"), Hash40::new("top"), 0, 0, 0, 0, 120, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_07"), false, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
        }
    }
    
    if slot_wrapped == 7 {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_08"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_08"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_08"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 110, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_08"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_08"), Hash40::new("top"), 0, 0, 0, 180, -110, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 180, -100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_08"), true, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_trace_08"), Hash40::new("top"), 0, 0, 0, 0, 120, 90, 1, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("packun_atk_hi_wind"), Hash40::new("top"), 0, 0, 0, 0, 100, 90, 1, true);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_trace_08"), false, true);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("packun_atk_hi_wind"), true, true);
        }
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(
        effect_attackhi3,
    );
}