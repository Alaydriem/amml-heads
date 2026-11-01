
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Restarted to delay deadcode elimination for space: ram */

ulonglong FUN_146bc4800(longlong param_1,longlong *param_2,undefined8 param_3)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  ulonglong uVar4;
  int iVar5;
  longlong *plVar6;
  ulonglong uVar7;
  undefined8 uVar8;
  longlong lVar9;
  ulonglong uVar10;
  longlong lVar11;
  longlong *plVar12;
  undefined **ppuVar13;
  undefined8 unaff_RBP;
  undefined **ppuVar14;
  longlong lVar15;
  longlong lVar16;
  uint unaff_R15D;
  bool bVar17;
  undefined **ppuVar18;
  undefined **ppuVar19;
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
  
  (*(code *)PTR__guard_dispatch_icall_14f99ef70)(*(undefined8 *)(param_1 + 0x58),&lStack_d0);
  lVar9 = lStack_c8;
  lVar16 = 0;
  if ((lStack_c8 != 0) && ((uStack_c0 & 0x3ffff) != 0x3ffff)) {
    iVar5 = *(int *)(lStack_c8 + 8);
    lVar16 = 0;
    do {
      if (iVar5 == 0) goto LAB_146bc49eb;
      LOCK();
      iVar2 = *(int *)(lStack_c8 + 8);
      bVar17 = iVar5 == iVar2;
      if (bVar17) {
        *(int *)(lStack_c8 + 8) = iVar5 + 1;
        iVar2 = iVar5;
      }
      iVar5 = iVar2;
      UNLOCK();
    } while (!bVar17);
    if (lStack_d0 == 0) {
LAB_146bc48eb:
      unaff_RBP = 0;
      lVar15 = lStack_d0;
    }
    else {
      uVar10 = (ulonglong)((uStack_c0 & 0x3ffff) >> 0xb);
      unaff_R15D = uStack_c0;
      if (((((ulonglong)(*(longlong *)(lStack_d0 + 0x118) - *(longlong *)(lStack_d0 + 0x110) >> 3)
             <= uVar10) ||
           (lVar16 = *(longlong *)(*(longlong *)(lStack_d0 + 0x110) + uVar10 * 8), lVar16 == 0)) ||
          (uVar3 = *(uint *)(lVar16 + (ulonglong)(uStack_c0 & 0x7ff) * 4),
          0x3fffe < (uStack_c0 & 0xfffc0000 ^ uVar3))) ||
         (*(ulonglong *)(lStack_d0 + 0x150) <= (ulonglong)(uVar3 & 0x3ffff))) goto LAB_146bc48eb;
      unaff_RBP = CONCAT71((int7)((ulonglong)unaff_RBP >> 8),1);
      lVar15 = lStack_d0 + 0x30;
    }
    if (lStack_c8 != 0) {
      LOCK();
      piVar1 = (int *)(lStack_c8 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_c8);
        LOCK();
        piVar1 = (int *)(lVar9 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar9);
        }
      }
    }
    if ((char)unaff_RBP != '\0') {
      plVar6 = (longlong *)
               (*(longlong *)(lVar15 + 0x48) +
               (ulonglong)
               (((uint)(*(int *)(lVar15 + 0x50) - (int)*(longlong *)(lVar15 + 0x48)) >> 3) - 1 &
               0xb845379) * 8);
      lVar9 = *(longlong *)(lVar15 + 0x68);
      lVar16 = 0;
      do {
        if (*plVar6 == -1) goto LAB_146bc49eb;
        lVar11 = *plVar6 * 0x20;
        plVar6 = (longlong *)(lVar9 + lVar11);
      } while (*(int *)(lVar9 + 8 + lVar11) != 0xb845379);
      if ((*(longlong *)(lVar15 + 0x70) != lVar9 + lVar11) &&
         (lVar9 = *(longlong *)(lVar9 + lVar11 + 0x10), lVar9 != 0)) {
        uVar10 = (ulonglong)((unaff_R15D & 0x3ffff) >> 0xb);
        if ((uVar10 < (ulonglong)(*(longlong *)(lVar9 + 0x10) - *(longlong *)(lVar9 + 8) >> 3)) &&
           ((lVar16 = *(longlong *)(*(longlong *)(lVar9 + 8) + uVar10 * 8), lVar16 != 0 &&
            (uVar3 = *(uint *)(lVar16 + (ulonglong)(unaff_R15D & 0x7ff) * 4),
            (unaff_R15D & 0xfffc0000 ^ uVar3) < 0x3ffff)))) {
          lVar16 = (ulonglong)(uVar3 & 0x7f) * 0x250 +
                   *(longlong *)(*(longlong *)(lVar9 + 0x50) + (ulonglong)((uVar3 & 0x3ff80) >> 4));
          goto LAB_146bc49eb;
        }
      }
    }
    lVar16 = 0;
  }
LAB_146bc49eb:
  if (lStack_c8 != 0) {
    LOCK();
    piVar1 = (int *)(lStack_c8 + 0xc);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    }
  }
  plVar6 = (longlong *)0x1528d5fb8;
  if (lVar16 != 0) {
    plVar6 = (longlong *)(lVar16 + 0xc0);
  }
  plVar12 = (longlong *)0x1528d5fb0;
  if (lVar16 != 0) {
    plVar12 = (longlong *)(lVar16 + 0xb8);
  }
  uVar10 = CONCAT71((int7)((ulonglong)unaff_RBP >> 8),1);
  if ((*param_2 != *plVar12) || (param_2[1] != *plVar6)) {
    if (*(char **)(param_1 + 0x480) == (char *)0x0) {
      _DAT_00000000 = 0xdeadc0de;
    }
    else if (**(char **)(param_1 + 0x480) == '\0') {
      _DAT_00000000 = 0xdeadc0de;
    }
    lVar9 = *(longlong *)(*(longlong *)(param_1 + 0x490) + 0x20);
    lVar16 = *(longlong *)(lVar9 + 0x1d8);
    uVar7 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    if ((lVar16 != 0) && (uVar4 = *(ulonglong *)(lVar9 + 0x1d8), uVar4 < uVar7)) {
      uVar7 = uVar4;
    }
    if ((longlong)uVar7 < *(longlong *)(param_1 + 0x408) * 0x100000) {
      uVar8 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
      puStack_48 = &UNK_14f8469d4;
      uStack_40 = 0;
      puStack_58 = &UNK_14f8469d4;
      uStack_50 = 0;
      puStack_68 = &UNK_14f8469d4;
      uStack_60 = 0;
      puStack_78 = &UNK_14f517faa;
      uStack_70 = 0xe;
      ppuVar19 = &puStack_48;
      ppuVar18 = &puStack_58;
      ppuVar13 = &puStack_78;
      ppuVar14 = &puStack_68;
    }
    else {
      if ((*(longlong *)(param_1 + 0x400) * 0x100000 <= (longlong)uVar7) ||
         (lVar9 = FUN_14298f4e0(param_3), lVar9 < *(longlong *)(param_1 + 0x410) * 0x400))
      goto LAB_146bc4b96;
      uVar8 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
      puStack_88 = &UNK_14f8469d4;
      uStack_80 = 0;
      puStack_98 = &UNK_14f8469d4;
      uStack_90 = 0;
      puStack_a8 = &UNK_14f8469d4;
      uStack_a0 = 0;
      puStack_b8 = &UNK_14f517f4e;
      uStack_b0 = 0x15;
      ppuVar19 = &puStack_88;
      ppuVar18 = &puStack_98;
      ppuVar13 = &puStack_b8;
      ppuVar14 = &puStack_a8;
    }
    uVar10 = 0;
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uVar8,ppuVar13,ppuVar14,0,0,ppuVar18,ppuVar19);
  }
LAB_146bc4b96:
  return uVar10 & 0xffffffff;
}

