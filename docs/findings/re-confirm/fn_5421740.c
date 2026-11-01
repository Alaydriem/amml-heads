
longlong * FUN_145421740(longlong param_1,longlong param_2,longlong *param_3)

{
  int *piVar1;
  int iVar2;
  longlong lVar3;
  longlong lVar4;
  code *pcVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  undefined *puVar9;
  longlong *plVar10;
  longlong alStack_78 [2];
  undefined *puStack_68;
  longlong lStack_60;
  longlong lStack_58;
  longlong lStack_50;
  longlong *plStack_40;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  plStack_40 = param_3;
  FUN_1454756c0(param_1 + 0x1a8,alStack_78,param_2);
  plVar10 = (longlong *)(alStack_78[0] + 0x58);
  FUN_145676fe0(plVar10);
  lVar3 = *(longlong *)(param_2 + 0x18);
  if (*(ulonglong *)(param_2 + 0x20) < 0x10) {
    puVar9 = (undefined *)(param_2 + 8);
  }
  else {
    puVar9 = *(undefined **)(param_2 + 8);
  }
  puStack_68 = &UNK_14f7ce9a9;
  if (lVar3 != 0) {
    puStack_68 = puVar9;
  }
  lStack_60 = 0x1d;
  if (lVar3 != 0) {
    lStack_60 = lVar3;
  }
  if (plStack_40[1] == 0) {
    lStack_50 = 0;
  }
  else {
    LOCK();
    piVar1 = (int *)(plStack_40[1] + 8);
    *piVar1 = *piVar1 + 1;
    UNLOCK();
    lStack_50 = plStack_40[1];
  }
  lStack_58 = *plStack_40;
  if (lStack_58 != 0) {
    FUN_145675890(plVar10,*(undefined8 *)(param_1 + 0x1d0),&lStack_58,&puStack_68);
    lVar3 = *(longlong *)(param_1 + 0x240);
    if (lVar3 != 0) {
      lVar4 = *plVar10;
      if (*(int *)(lVar4 + 0x18) - 0x75U < 0x1c) {
        uVar6 = FUN_145a490d0(*(int *)(lVar4 + 0x18),*(undefined4 *)(lVar4 + 0x10),
                              *(undefined4 *)(lVar4 + 0x14),1);
      }
      else {
        iVar8 = *(int *)(lVar4 + 0x10);
        iVar7 = 0;
        if ((iVar8 != 0) && (iVar2 = *(int *)(lVar4 + 0x14), iVar2 != 0)) {
          iVar7 = func_0x000145a4c660();
          iVar7 = iVar7 * iVar2 * iVar8;
        }
        iVar8 = 6;
        if (*(char *)(lVar4 + 0x1d) != '\x01') {
          iVar8 = *(int *)(lVar4 + 0x20);
        }
        uVar6 = iVar7 * iVar8 * ((uint)(*(char *)(lVar4 + 0x1d) == '\x03') * 5 + 1);
      }
      *(longlong *)(lVar3 + 8) = *(longlong *)(lVar3 + 8) + (ulonglong)uVar6;
      FUN_141127050(lVar3,param_2);
    }
    lVar3 = plStack_40[1];
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
    return plVar10;
  }
  FUN_140078ed0();
  pcVar5 = (code *)swi(3);
  plVar10 = (longlong *)(*pcVar5)();
  return plVar10;
}

