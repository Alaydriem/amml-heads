
longlong * FUN_145420ec0(longlong param_1,longlong param_2,longlong *param_3,longlong *param_4)

{
  int *piVar1;
  int iVar2;
  longlong lVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  char cVar8;
  char cVar9;
  uint uVar10;
  int iVar11;
  int iVar12;
  longlong lVar13;
  longlong *plVar14;
  undefined8 uVar15;
  undefined1 auStack_140 [24];
  undefined1 auStack_128 [8];
  longlong lStack_120;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined1 uStack_a8;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined2 uStack_78;
  code *pcStack_70;
  longlong lStack_68;
  undefined8 uStack_60;
  longlong lStack_58;
  uint uStack_4c;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  FUN_1454756c0(param_1 + 0x1a8,&uStack_f8,param_2);
  plVar14 = (longlong *)(uStack_f8 + 0x58);
  FUN_145676fe0(plVar14);
  if (param_4[1] == 0) {
    lVar3 = *(longlong *)(param_2 + 0x18);
    if (*(ulonglong *)(param_2 + 0x20) < 0x10) {
      lVar13 = param_2 + 8;
    }
    else {
      lVar13 = *(longlong *)(param_2 + 8);
    }
    *param_4 = lVar13;
    param_4[1] = lVar3;
    if (lVar3 == 0) {
      *param_4 = (longlong)&UNK_14f7ce9a9;
      param_4[1] = 0x1d;
    }
  }
  lStack_58 = param_2;
  if (*param_3 != 0) {
    cVar8 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    lVar3 = param_3[3];
    cVar9 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    uStack_4c = *(uint *)((longlong)param_3 + 0x24);
    uVar10 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    if (((((int)lVar3 != 0) && (cVar8 == '\0')) && (cVar9 != '\0')) &&
       (uVar15 = 1, uStack_4c <= uVar10)) goto LAB_145421119;
  }
  uStack_88 = 0;
  uStack_80 = 0;
  uStack_78 = 0;
  pcStack_70 = FUN_1403c7b10;
  lStack_68 = 0;
  uStack_60 = 0;
  FUN_1419950d0(auStack_140,&uStack_88,4,&UNK_14f24b8d0,0x9e,0);
  FUN_145a4c000(auStack_128,&uStack_88);
  FUN_144df8eb0(&uStack_f8,auStack_128,1);
  uVar4 = (undefined4)uStack_f8;
  uVar5 = uStack_f8._4_4_;
  uVar6 = (undefined4)uStack_f0;
  uVar7 = uStack_f0._4_4_;
  uStack_f8 = 0;
  uStack_f0 = 0;
  lVar3 = param_3[1];
  *(undefined4 *)param_3 = uVar4;
  *(undefined4 *)((longlong)param_3 + 4) = uVar5;
  *(undefined4 *)(param_3 + 1) = uVar6;
  *(undefined4 *)((longlong)param_3 + 0xc) = uVar7;
  if (lVar3 != 0) {
    LOCK();
    piVar1 = (int *)(lVar3 + 8);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar3);
      LOCK();
      piVar1 = (int *)(lVar3 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar3);
      }
    }
  }
  lVar3 = uStack_f0;
  *(undefined4 *)(param_3 + 8) = uStack_b8;
  *(undefined4 *)((longlong)param_3 + 0x44) = uStack_b4;
  *(undefined4 *)(param_3 + 9) = uStack_b0;
  *(undefined4 *)((longlong)param_3 + 0x4c) = uStack_ac;
  *(undefined4 *)(param_3 + 6) = uStack_c8;
  *(undefined4 *)((longlong)param_3 + 0x34) = uStack_c4;
  *(undefined4 *)(param_3 + 7) = uStack_c0;
  *(undefined4 *)((longlong)param_3 + 0x3c) = uStack_bc;
  *(undefined4 *)(param_3 + 4) = uStack_d8;
  *(undefined4 *)((longlong)param_3 + 0x24) = uStack_d4;
  *(undefined4 *)(param_3 + 5) = uStack_d0;
  *(undefined4 *)((longlong)param_3 + 0x2c) = uStack_cc;
  *(undefined4 *)(param_3 + 2) = uStack_e8;
  *(undefined4 *)((longlong)param_3 + 0x14) = uStack_e4;
  *(undefined4 *)(param_3 + 3) = uStack_e0;
  *(undefined4 *)((longlong)param_3 + 0x1c) = uStack_dc;
  *(undefined1 *)(param_3 + 10) = uStack_a8;
  if (uStack_f0 != 0) {
    LOCK();
    piVar1 = (int *)(uStack_f0 + 8);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_f0);
      LOCK();
      piVar1 = (int *)(lVar3 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar3);
      }
    }
  }
  if (lStack_120 != 0) {
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
  }
  if (lStack_68 != 0) {
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
  }
  uVar15 = 0;
LAB_145421119:
  uStack_98 = (undefined4)*param_4;
  uStack_94 = *(undefined4 *)((longlong)param_4 + 4);
  uStack_90 = (undefined4)param_4[1];
  uStack_8c = *(undefined4 *)((longlong)param_4 + 0xc);
  FUN_145673bb0(plVar14,*(undefined8 *)(param_1 + 0x1d0),param_3,uVar15,&uStack_98,0);
  lVar3 = *(longlong *)(param_1 + 0x240);
  if (lVar3 != 0) {
    lVar13 = *plVar14;
    if (*(int *)(lVar13 + 0x18) - 0x75U < 0x1c) {
      uVar10 = FUN_145a490d0(*(int *)(lVar13 + 0x18),*(undefined4 *)(lVar13 + 0x10),
                             *(undefined4 *)(lVar13 + 0x14),1);
    }
    else {
      iVar12 = *(int *)(lVar13 + 0x10);
      iVar11 = 0;
      if ((iVar12 != 0) && (iVar2 = *(int *)(lVar13 + 0x14), iVar2 != 0)) {
        iVar11 = func_0x000145a4c660();
        iVar11 = iVar11 * iVar2 * iVar12;
      }
      iVar12 = 6;
      if (*(char *)(lVar13 + 0x1d) != '\x01') {
        iVar12 = *(int *)(lVar13 + 0x20);
      }
      uVar10 = iVar11 * iVar12 * ((uint)(*(char *)(lVar13 + 0x1d) == '\x03') * 5 + 1);
    }
    *(longlong *)(lVar3 + 8) = *(longlong *)(lVar3 + 8) + (ulonglong)uVar10;
    FUN_141127050(lVar3,lStack_58);
  }
  return plVar14;
}

