# smush_extra_slots_effect_fix
A skyline plugin for smash ultimate that fixes severe effect glitches on added slots (c08+) for certain chars.

# Dependencies
* nro hook (https://github.com/ultimate-research/nro-hook-plugin)
* smashline hook (https://github.com/blu-dev/smashline_hook)

# Why do some chars effects glitch out after slot 8?
* Some fighter's moves choose an effect by specifying the effect by name but then offset the specified effect with the slot. Since the devs only had 8 slots in mind, slots above 8 will begin to load incorrect effects from further along in the effect file.

### Example
#### Fox's Effects (from fox's `.eff` effect file)
* ![image](https://user-images.githubusercontent.com/77519735/230828848-9ecfff3a-0144-4ae9-b009-a050c8041a27.png)
* Fox's Down-tilt effect script calls `EFFECT_FOLLOW_arg11(Hash40::new("fox_tail_attack_01"),..., *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);`, and this will in vanilla smash ensure that the correct effect is chosen by offseting `fox_tail_attack_01` with the slot number (`*FIGHTER_INSTANCE_WORK_ID_INT_COLOR`).
* So, if i pick the 8th fox, the index is 7 and `EFFECT_FOLLOW_arg11` will call `fox_tail_attack_01` with an offset of `7`, correctly loading `FOX_TAIL_ATTACK_08` from the `.eff` file.
* However if i try to play on the 9th slot, which normally does not exist in vanilla smash, it will load with an offset of `8` and incorrectly load `FOX_BLASTER_SPIN` instead

# The "solution"
* The index just wraps around, so slot 9 loads slot 1, 16 loads 8, 24 loads 8, etc....
* Not the perfect solution, but at least the colors are just mismatched rather than loading the completely wrong effect.

# Chars fixed by this plugin
* sonic
* fox
* yoshi 
* pirhana plant
* mewtwo
* dark samus
* duckhunt

# Disclaimer:
* This is pretty much just copy-pasted from Wuboy's dumped smashline scripts, just with the slot number "fixed".

# Special Thanks:
* Wuboy for providing the dumped smashline scripts (https://github.com/WuBoytH/SSBU-Dumped-Scripts/tree/main/smashline)
* 010 editor was used in the posted screenshot with foxes effects (https://www.sweetscape.com/)

