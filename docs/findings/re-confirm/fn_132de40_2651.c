
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_14132de40(longlong param_1,longlong *param_2,undefined8 param_3)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  ulonglong uVar4;
  char cVar5;
  int iVar6;
  longlong *plVar7;
  ulonglong uVar8;
  undefined8 uVar9;
  longlong lVar10;
  ulonglong uVar11;
  longlong lVar12;
  longlong *plVar13;
  undefined **ppuVar14;
  undefined8 unaff_RBP;
  undefined **ppuVar15;
  longlong lVar16;
  longlong lVar17;
  uint unaff_R15D;
  bool bVar18;
  undefined *in_stack_ffffffffffffff18;
  undefined **ppuVar19;
  undefined **ppuVar20;
  longlong lStack_d0;
  longlong lStack_c8;
  uint uStack_c0;
  undefined *puStack_b8;
  undefined8 uStack_b0;
  undefined *puStack_a8;
  undefined8 uStack_a0;
  undefined *puStack_98;
  undefined8 uStack_90;
  undefined *puStack_88;
  undefined8 uStack_80;
  undefined *puStack_78;
  undefined8 uStack_70;
  undefined *puStack_68;
  undefined8 uStack_60;
  undefined *puStack_58;
  undefined8 uStack_50;
  undefined *puStack_48;
  undefined8 uStack_40;
  
  (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(*(undefined8 *)(param_1 + 0x58),&lStack_d0);
  lVar10 = lStack_c8;
  lVar17 = 0;
  if ((lStack_c8 != 0) && ((uStack_c0 & 0x3ffff) != 0x3ffff)) {
    iVar6 = *(int *)(lStack_c8 + 8);
    lVar17 = 0;
    do {
      if (iVar6 == 0) goto LAB_14132e02b;
      LOCK();
      iVar2 = *(int *)(lStack_c8 + 8);
      bVar18 = iVar6 == iVar2;
      if (bVar18) {
        *(int *)(lStack_c8 + 8) = iVar6 + 1;
        iVar2 = iVar6;
      }
      iVar6 = iVar2;
      UNLOCK();
    } while (!bVar18);
    if (lStack_d0 == 0) {
LAB_14132df2b:
      unaff_RBP = 0;
      lVar16 = lStack_d0;
    }
    else {
      uVar11 = (ulonglong)((uStack_c0 & 0x3ffff) >> 0xb);
      unaff_R15D = uStack_c0;
      if (((((ulonglong)(*(longlong *)(lStack_d0 + 0x118) - *(longlong *)(lStack_d0 + 0x110) >> 3)
             <= uVar11) ||
           (lVar17 = *(longlong *)(*(longlong *)(lStack_d0 + 0x110) + uVar11 * 8), lVar17 == 0)) ||
          (uVar3 = *(uint *)(lVar17 + (ulonglong)(uStack_c0 & 0x7ff) * 4),
          0x3fffe < (uStack_c0 & 0xfffc0000 ^ uVar3))) ||
         (*(ulonglong *)(lStack_d0 + 0x150) <= (ulonglong)(uVar3 & 0x3ffff))) goto LAB_14132df2b;
      unaff_RBP = CONCAT71((int7)((ulonglong)unaff_RBP >> 8),1);
      lVar16 = lStack_d0 + 0x30;
    }
    if (lStack_c8 != 0) {
      LOCK();
      piVar1 = (int *)(lStack_c8 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lStack_c8);
        LOCK();
        piVar1 = (int *)(lVar10 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar10);
        }
      }
    }
    if ((char)unaff_RBP != '\0') {
      plVar7 = (longlong *)
               (*(longlong *)(lVar16 + 0x48) +
               (ulonglong)
               (((uint)(*(int *)(lVar16 + 0x50) - (int)*(longlong *)(lVar16 + 0x48)) >> 3) - 1 &
               0xb845379) * 8);
      lVar10 = *(longlong *)(lVar16 + 0x68);
      lVar17 = 0;
      do {
        if (*plVar7 == -1) goto LAB_14132e02b;
        lVar12 = *plVar7 * 0x20;
        plVar7 = (longlong *)(lVar10 + lVar12);
      } while (*(int *)(lVar10 + 8 + lVar12) != 0xb845379);
      if ((*(longlong *)(lVar16 + 0x70) != lVar10 + lVar12) &&
         (lVar10 = *(longlong *)(lVar10 + lVar12 + 0x10), lVar10 != 0)) {
        uVar11 = (ulonglong)((unaff_R15D & 0x3ffff) >> 0xb);
        if ((uVar11 < (ulonglong)(*(longlong *)(lVar10 + 0x10) - *(longlong *)(lVar10 + 8) >> 3)) &&
           ((lVar17 = *(longlong *)(*(longlong *)(lVar10 + 8) + uVar11 * 8), lVar17 != 0 &&
            (uVar3 = *(uint *)(lVar17 + (ulonglong)(unaff_R15D & 0x7ff) * 4),
            (unaff_R15D & 0xfffc0000 ^ uVar3) < 0x3ffff)))) {
          lVar17 = (ulonglong)(uVar3 & 0x7f) * 0x250 +
                   *(longlong *)(*(longlong *)(lVar10 + 0x50) + (ulonglong)((uVar3 & 0x3ff80) >> 4))
          ;
          goto LAB_14132e02b;
        }
      }
    }
    lVar17 = 0;
  }
LAB_14132e02b:
  if (lStack_c8 != 0) {
    LOCK();
    piVar1 = (int *)(lStack_c8 + 0xc);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
    }
  }
  plVar7 = (longlong *)0x151d325b8;
  if (lVar17 != 0) {
    plVar7 = (longlong *)(lVar17 + 0xc0);
  }
  plVar13 = (longlong *)0x151d325b0;
  if (lVar17 != 0) {
    plVar13 = (longlong *)(lVar17 + 0xb8);
  }
  uVar11 = CONCAT71((int7)((ulonglong)unaff_RBP >> 8),1);
  if ((*param_2 != *plVar13) || (param_2[1] != *plVar7)) {
    if (*(longlong *)(param_1 + 0x428) == 0) {
      in_stack_ffffffffffffff18 = &UNK_14ed5b2f3;
      cVar5 = FUN_140289fe0(&UNK_14ec0360b,&UNK_14ec4122d,300,&UNK_14ec489d8,&UNK_14ed5b2f3);
      if (cVar5 != '\0') {
        _DAT_00000000 = 0xdeadc0de;
      }
    }
    if (**(char **)(param_1 + 0x428) == '\0') {
      in_stack_ffffffffffffff18 = &UNK_14ed5b2f3;
      cVar5 = FUN_140289fe0(&UNK_14ec89e5e,&UNK_14ec87783,0x130,&UNK_14ec489d8,&UNK_14ed5b2f3);
      if (cVar5 != '\0') {
        _DAT_00000000 = 0xdeadc0de;
      }
    }
    lVar10 = *(longlong *)(*(longlong *)(param_1 + 0x438) + 0x20);
    lVar17 = *(longlong *)(lVar10 + 0x1d8);
    uVar8 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
    if ((lVar17 != 0) && (uVar4 = *(ulonglong *)(lVar10 + 0x1d8), uVar4 < uVar8)) {
      uVar8 = uVar4;
    }
    if ((longlong)uVar8 < *(longlong *)(param_1 + 0x3b0) * 0x100000) {
      uVar9 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
      puStack_48 = &UNK_14efacaeb;
      uStack_40 = 0;
      puStack_58 = &UNK_14efacaeb;
      uStack_50 = 0;
      puStack_68 = &UNK_14efacaeb;
      uStack_60 = 0;
      puStack_78 = &UNK_14ebb585a;
      uStack_70 = 0xe;
      ppuVar20 = &puStack_48;
      ppuVar19 = &puStack_58;
      ppuVar14 = &puStack_78;
      ppuVar15 = &puStack_68;
    }
    else {
      if ((*(longlong *)(param_1 + 0x3a8) * 0x100000 <= (longlong)uVar8) ||
         (lVar10 = FUN_142650180(param_3), lVar10 < *(longlong *)(param_1 + 0x3b8) * 0x400))
      goto LAB_14132e242;
      uVar9 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
      puStack_88 = &UNK_14efacaeb;
      uStack_80 = 0;
      puStack_98 = &UNK_14efacaeb;
      uStack_90 = 0;
      puStack_a8 = &UNK_14efacaeb;
      uStack_a0 = 0;
      puStack_b8 = &UNK_14ebb57fe;
      uStack_b0 = 0x15;
      ppuVar20 = &puStack_88;
      ppuVar19 = &puStack_98;
      ppuVar14 = &puStack_b8;
      ppuVar15 = &puStack_a8;
    }
    uVar11 = 0;
    (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)
              (uVar9,ppuVar14,ppuVar15,0,(ulonglong)in_stack_ffffffffffffff18 & 0xffffffffffffff00,
               ppuVar19,ppuVar20);
  }
LAB_14132e242:
  return uVar11 & 0xffffffff;
}

