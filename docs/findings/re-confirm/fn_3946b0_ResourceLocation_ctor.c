
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

byte * FUN_1403946b0(byte *param_1,undefined8 *param_2)

{
  undefined8 uVar1;
  code *pcVar2;
  longlong lVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  byte *pbVar6;
  ulonglong uVar7;
  byte *pbVar8;
  
  param_1[0] = 0;
  param_1[1] = 0;
  param_1[2] = 0;
  param_1[3] = 0;
  uVar1 = *param_2;
  uVar4 = param_2[1];
  param_1[0x18] = 0;
  param_1[0x19] = 0;
  param_1[0x1a] = 0;
  param_1[0x1b] = 0;
  param_1[0x1c] = 0;
  param_1[0x1d] = 0;
  param_1[0x1e] = 0;
  param_1[0x1f] = 0;
  param_1[0x20] = 0;
  param_1[0x21] = 0;
  param_1[0x22] = 0;
  param_1[0x23] = 0;
  param_1[0x24] = 0;
  param_1[0x25] = 0;
  param_1[0x26] = 0;
  param_1[0x27] = 0;
  param_1[8] = 0;
  param_1[9] = 0;
  param_1[10] = 0;
  param_1[0xb] = 0;
  param_1[0xc] = 0;
  param_1[0xd] = 0;
  param_1[0xe] = 0;
  param_1[0xf] = 0;
  param_1[0x10] = 0;
  param_1[0x11] = 0;
  param_1[0x12] = 0;
  param_1[0x13] = 0;
  param_1[0x14] = 0;
  param_1[0x15] = 0;
  param_1[0x16] = 0;
  param_1[0x17] = 0;
  if ((longlong)uVar4 < 0) {
    FUN_140075720();
    pcVar2 = (code *)swi(3);
    pbVar6 = (byte *)(*pcVar2)();
    return pbVar6;
  }
  pbVar6 = param_1 + 8;
  uVar7 = 0xf;
  pbVar8 = pbVar6;
  if (0xf < uVar4) {
    uVar5 = uVar4 | 0xf;
    uVar7 = 0x16;
    if (0x16 < uVar5) {
      uVar7 = uVar5;
    }
    if (uVar5 < 0xfff) {
      pbVar8 = (byte *)(*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,uVar7 + 1);
      if (pbVar8 == (byte *)0x0) {
        FUN_1409bc820(uVar7 + 1);
        pbVar8 = (byte *)0x0;
      }
    }
    else {
      lVar3 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,uVar7 + 0x28);
      if (lVar3 == 0) {
        FUN_1409bc820(uVar7 + 0x28);
      }
      pbVar8 = (byte *)(lVar3 + 0x27U & 0xffffffffffffffe0);
      *(longlong *)(pbVar8 + -8) = lVar3;
    }
    *(byte **)pbVar6 = pbVar8;
  }
  *(ulonglong *)(param_1 + 0x18) = uVar4;
  *(ulonglong *)(param_1 + 0x20) = uVar7;
  FUN_14eec62c0(pbVar8,uVar1,uVar4);
  pbVar8[uVar4] = 0;
  if (0xf < *(ulonglong *)(param_1 + 0x20)) {
    pbVar6 = *(byte **)(param_1 + 8);
  }
  if ((pbVar6 == (byte *)0x0) || (*pbVar6 == 0)) {
    uVar4 = 0;
  }
  else {
    uVar7 = FUN_14eec6970(pbVar6);
    uVar4 = 0xcbf29ce484222325;
    if (uVar7 != 0) {
      uVar5 = (ulonglong)((uint)uVar7 & 7);
      if (uVar7 < 8) {
        uVar4 = 0xcbf29ce484222325;
      }
      else {
        uVar7 = uVar7 & 0xfffffffffffffff8;
        uVar4 = 0xcbf29ce484222325;
        do {
          uVar4 = (((((((uVar4 * 0x100000001b3 ^ (ulonglong)*pbVar6) * 0x100000001b3 ^
                       (ulonglong)pbVar6[1]) * 0x100000001b3 ^ (ulonglong)pbVar6[2]) * 0x100000001b3
                     ^ (ulonglong)pbVar6[3]) * 0x100000001b3 ^ (ulonglong)pbVar6[4]) * 0x100000001b3
                   ^ (ulonglong)pbVar6[5]) * 0x100000001b3 ^ (ulonglong)pbVar6[6]) * 0x100000001b3 ^
                  (ulonglong)pbVar6[7];
          pbVar6 = pbVar6 + 8;
          uVar7 = uVar7 - 8;
        } while (uVar7 != 0);
        if (uVar5 == 0) goto LAB_140394874;
      }
      uVar7 = 0;
      do {
        uVar4 = uVar4 * 0x100000001b3 ^ (ulonglong)pbVar6[uVar7];
        uVar7 = uVar7 + 1;
      } while (uVar5 != uVar7);
    }
  }
LAB_140394874:
  *(ulonglong *)(param_1 + 0x28) = uVar4;
  *(ulonglong *)(param_1 + 0x30) =
       ((ulonglong)*param_1 ^ 0xcbf29ce484222325) * 0x100000001b3 ^ uVar4;
  return param_1;
}

