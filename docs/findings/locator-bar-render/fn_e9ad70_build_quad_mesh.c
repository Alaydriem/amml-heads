
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140e9ad70(undefined8 param_1,undefined8 param_2,ulonglong param_3,undefined4 *param_4)

{
  code *pcVar1;
  longlong lVar2;
  ulonglong uVar3;
  undefined8 auStack_2c8 [76];
  undefined *puStack_68;
  undefined8 uStack_60;
  longlong lStack_58;
  undefined8 *puStack_50;
  undefined8 *puStack_48;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  lVar2 = _DAT_15294c2f0;
  if ((_DAT_15294c2f0 == 0) &&
     (lVar2 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,1), lVar2 == 0)) {
    FUN_1409bc820(1);
  }
  _DAT_15294c2f0 = lVar2;
  lVar2 = _DAT_15294c2f0;
  lStack_58 = 0;
  puStack_50 = (undefined8 *)0x0;
  puStack_48 = (undefined8 *)0x0;
  auStack_2c8[0] = 1;
  FUN_1402fb1c0(&lStack_58,auStack_2c8);
  auStack_2c8[0] = 0;
  if (puStack_50 == puStack_48) {
    FUN_1400a4ec0(&lStack_58,puStack_50,auStack_2c8);
  }
  else {
    *puStack_50 = 0;
    puStack_50 = puStack_50 + 1;
  }
  uVar3 = param_3;
  if (param_4 == (undefined4 *)0x0) {
    FUN_1424e6230(lVar2,&lStack_58,_DAT_14f0883d8,_DAT_14f0883d8,param_3,param_2);
  }
  else {
    FUN_1424e6230(lVar2,&lStack_58,*param_4,param_4[1],param_3,param_2);
  }
  puStack_68 = &UNK_14f56cc32;
  uStack_60 = 8;
  FUN_145418a70(param_3,auStack_2c8,0,&puStack_68,uVar3 & 0xffffffffffff0000);
  FUN_144de7fa0(param_1,auStack_2c8);
  FUN_144dea7c0(auStack_2c8);
  if (lStack_58 != 0) {
    uVar3 = (longlong)puStack_48 - lStack_58;
    lVar2 = lStack_58;
    if (0xfff < uVar3) {
      lVar2 = *(longlong *)(lStack_58 + -8);
      if (0x1f < (ulonglong)((lStack_58 + -8) - lVar2)) {
        _invoke_watson(0,0,0,0,0);
        pcVar1 = (code *)swi(3);
        (*pcVar1)();
        return;
      }
      uVar3 = uVar3 + 0x27;
    }
    FUN_14e934b00(lVar2,uVar3);
  }
  return;
}

