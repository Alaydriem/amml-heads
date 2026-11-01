
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Restarted to delay deadcode elimination for space: ram */

undefined8 *
FUN_1457e1da0(undefined8 *param_1,longlong param_2,undefined8 param_3,undefined8 param_4)

{
  int *piVar1;
  longlong lVar2;
  longlong lVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  longlong lVar9;
  undefined1 uVar10;
  uint uVar11;
  
  *param_1 = &UNK_14f261b60;
  param_1[1] = 0;
  param_1[2] = 0;
  param_1[3] = param_3;
  param_1[4] = param_4;
  param_1[5] = param_2;
  uVar7 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_3);
  param_1[6] = uVar7;
  (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_3);
  uVar7 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_3);
  param_1[9] = uVar7;
  uVar7 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_3);
  param_1[10] = uVar7;
  uVar7 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_3);
  param_1[0xb] = uVar7;
  uVar7 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_3);
  param_1[0xc] = uVar7;
  *(undefined1 *)(param_1 + 0x14) = 0;
  param_1[0xd] = 0;
  param_1[0xe] = 0;
  param_1[0xf] = 0;
  param_1[0x10] = 0;
  *(undefined8 *)((longlong)param_1 + 0x81) = 0;
  *(undefined8 *)((longlong)param_1 + 0x89) = 0;
  puVar8 = (undefined8 *)(*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,0x238);
  if (puVar8 == (undefined8 *)0x0) {
    FUN_1409bc820(0x238);
  }
  puVar8[6] = 0;
  puVar8[7] = 0;
  puVar8[4] = 0;
  puVar8[5] = 0;
  puVar8[2] = 0;
  puVar8[3] = 0;
  *puVar8 = 0;
  puVar8[1] = 0;
  uVar6 = uRam000000015295c744;
  uVar5 = uRam000000015295c740;
  uVar4 = uRam000000015295c73c;
  *(undefined4 *)(puVar8 + 7) = _DAT_15295c738;
  *(undefined4 *)((longlong)puVar8 + 0x3c) = uVar4;
  *(undefined4 *)(puVar8 + 8) = uVar5;
  *(undefined4 *)((longlong)puVar8 + 0x44) = uVar6;
  puVar8[9] = _DAT_15295c748;
  FUN_14eec62d0(puVar8 + 10,0,0x1e8);
  param_1[0x15] = puVar8;
  uVar11 = 0;
  if ((byte)*(uint *)(param_2 + 0x74) < 4) {
    uVar11 = *(uint *)(param_2 + 0x74);
  }
  param_1[0x16] = (ulonglong)*(uint *)(param_2 + 0x120 + (ulonglong)(uVar11 & 0xff) * 4);
  *(undefined4 *)(param_1 + 0x17) = 0x10101;
  *(undefined2 *)((longlong)param_1 + 0xbc) = 0;
  lVar9 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_3);
  if (lVar9 != 0) {
    lVar2 = *(longlong *)(lVar9 + 0x480);
    lVar3 = param_1[0x15];
    *(undefined4 *)(lVar3 + 0x34) = *(undefined4 *)(lVar2 + 0x654);
    *(undefined8 *)(lVar3 + 0x2c) = *(undefined8 *)(lVar2 + 0x64c);
    lVar2 = *(longlong *)(lVar9 + 0x480);
    lVar3 = param_1[0x15];
    *(undefined4 *)(lVar3 + 0x28) = *(undefined4 *)(lVar2 + 0x660);
    *(undefined8 *)(lVar3 + 0x20) = *(undefined8 *)(lVar2 + 0x658);
    if (*(char **)(lVar9 + 0x3c0) == (char *)0x0) {
      _DAT_00000000 = 0xdeadc0de;
    }
    else if (**(char **)(lVar9 + 0x3c0) == '\0') {
      _DAT_00000000 = 0xdeadc0de;
    }
    param_1[0xf] = *(undefined8 *)(lVar9 + 0x3d0);
    puVar8 = *(undefined8 **)(lVar9 + 0xb28);
    if ((puVar8 == (undefined8 *)0x0) || (*(char *)(puVar8 + 2) == '\0')) {
      uVar7 = 0;
    }
    else {
      uVar7 = *puVar8;
    }
    param_1[0x10] = uVar7;
    if (*(longlong *)(lVar9 + 0xb28) == 0) {
      uVar7 = 0;
    }
    else {
      uVar7 = *(undefined8 *)(*(longlong *)(lVar9 + 0xb28) + 8);
    }
    param_1[0x11] = uVar7;
    if (*(longlong *)(lVar9 + 0xb28) == 0) {
      uVar10 = 0;
    }
    else {
      uVar10 = *(undefined1 *)(*(longlong *)(lVar9 + 0xb28) + 0x11);
    }
    *(undefined1 *)(param_1 + 0x12) = uVar10;
    lVar2 = *(longlong *)(lVar9 + 0x98);
    if (lVar2 == 0) {
      uVar7 = 0;
    }
    else {
      uVar7 = *(undefined8 *)(lVar9 + 0x90);
      LOCK();
      *(int *)(lVar2 + 0xc) = *(int *)(lVar2 + 0xc) + 1;
      UNLOCK();
    }
    param_1[0xd] = uVar7;
    lVar9 = param_1[0xe];
    param_1[0xe] = lVar2;
    if (lVar9 != 0) {
      LOCK();
      piVar1 = (int *)(lVar9 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
      }
    }
  }
  return param_1;
}

