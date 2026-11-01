
/* WARNING: Removing unreachable block (ram,0x00014134a115) */
/* WARNING: Removing unreachable block (ram,0x00014134a11b) */
/* WARNING: Removing unreachable block (ram,0x00014134a139) */
/* WARNING: Removing unreachable block (ram,0x00014134a14c) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141348a10(longlong param_1,undefined8 param_2,longlong param_3)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  longlong lVar6;
  longlong lVar7;
  undefined8 uVar8;
  undefined *puVar9;
  code *pcVar10;
  undefined8 ******ppppppuVar11;
  char cVar12;
  char cVar13;
  int iVar14;
  longlong *plVar15;
  ulonglong uVar16;
  undefined8 uVar17;
  undefined8 *puVar18;
  longlong lVar19;
  longlong lVar20;
  longlong *plVar21;
  longlong lVar22;
  char *pcVar23;
  char *******pppppppcVar24;
  undefined8 *******pppppppuVar25;
  longlong lVar26;
  uint unaff_EBX;
  longlong lVar27;
  longlong lVar28;
  longlong lVar29;
  bool bVar30;
  undefined8 ******ppppppuStack_260;
  char ******ppppppcStack_258;
  undefined8 uStack_250;
  ulonglong uStack_248;
  undefined8 uStack_240;
  longlong lStack_230;
  undefined8 uStack_228;
  undefined4 uStack_220;
  uint uStack_210;
  char *pcStack_1b8;
  longlong lStack_1b0;
  undefined8 uStack_1a8;
  ulonglong uStack_1a0;
  uint uStack_198;
  uint uStack_188;
  undefined4 uStack_178;
  uint uStack_168;
  undefined *puStack_110;
  undefined8 uStack_108;
  longlong lStack_100;
  longlong lStack_f8;
  longlong lStack_f0;
  undefined8 uStack_e8;
  longlong lStack_d8;
  undefined8 uStack_d0;
  longlong lStack_c8;
  longlong lStack_b8;
  undefined8 ******ppppppuStack_b0;
  char **ppcStack_a8;
  ulonglong uStack_a0;
  ulonglong uStack_98;
  undefined *puStack_90;
  undefined8 ******ppppppuStack_88;
  char ******ppppppcStack_80;
  undefined8 ******ppppppuStack_78;
  ulonglong uStack_70;
  ulonglong uStack_68;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  if (*(char **)(param_1 + 600) == (char *)0x0) {
    return;
  }
  if (**(char **)(param_1 + 600) != '\x01') {
    return;
  }
  lStack_b8 = param_3;
  cVar12 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
  if (cVar12 != '\0') {
    return;
  }
  (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(*(undefined8 *)(param_1 + 0x58),&pcStack_1b8);
  lVar19 = lStack_1b0;
  lVar27 = 0;
  if ((lStack_1b0 == 0) || (((uint)uStack_1a8 & 0x3ffff) == 0x3ffff)) goto LAB_141348c45;
  iVar14 = *(int *)(lStack_1b0 + 8);
  lVar27 = 0;
  do {
    if (iVar14 == 0) goto LAB_141348c45;
    LOCK();
    iVar2 = *(int *)(lStack_1b0 + 8);
    bVar30 = iVar14 == iVar2;
    if (bVar30) {
      *(int *)(lStack_1b0 + 8) = iVar14 + 1;
      iVar2 = iVar14;
    }
    iVar14 = iVar2;
    UNLOCK();
  } while (!bVar30);
  if (pcStack_1b8 == (char *)0x0) {
LAB_141348b55:
    bVar30 = false;
    pcVar23 = pcStack_1b8;
  }
  else {
    uVar16 = (ulonglong)(((uint)uStack_1a8 & 0x3ffff) >> 0xb);
    unaff_EBX = (uint)uStack_1a8;
    if (((((ulonglong)(*(longlong *)(pcStack_1b8 + 0x118) - *(longlong *)(pcStack_1b8 + 0x110) >> 3)
           <= uVar16) ||
         (lVar27 = *(longlong *)(*(longlong *)(pcStack_1b8 + 0x110) + uVar16 * 8), lVar27 == 0)) ||
        (uVar3 = *(uint *)(lVar27 + (ulonglong)((uint)uStack_1a8 & 0x7ff) * 4),
        0x3fffe < ((uint)uStack_1a8 & 0xfffc0000 ^ uVar3))) ||
       (*(ulonglong *)(pcStack_1b8 + 0x150) <= (ulonglong)(uVar3 & 0x3ffff))) goto LAB_141348b55;
    bVar30 = true;
    pcVar23 = pcStack_1b8 + 0x30;
  }
  if (lStack_1b0 != 0) {
    LOCK();
    piVar1 = (int *)(lStack_1b0 + 8);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lStack_1b0);
      LOCK();
      piVar1 = (int *)(lVar19 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar19);
      }
    }
  }
  if (bVar30) {
    plVar15 = (longlong *)
              (*(longlong *)(pcVar23 + 0x48) +
              (ulonglong)
              (((uint)(*(int *)(pcVar23 + 0x50) - (int)*(longlong *)(pcVar23 + 0x48)) >> 3) - 1 &
              0xb845379) * 8);
    lVar19 = *(longlong *)(pcVar23 + 0x68);
    lVar27 = 0;
    do {
      if (*plVar15 == -1) goto LAB_141348c45;
      lVar20 = *plVar15 * 0x20;
      plVar15 = (longlong *)(lVar19 + lVar20);
    } while (*(int *)(lVar19 + 8 + lVar20) != 0xb845379);
    if ((*(longlong *)(pcVar23 + 0x70) != lVar19 + lVar20) &&
       (lVar19 = *(longlong *)(lVar19 + lVar20 + 0x10), lVar19 != 0)) {
      uVar16 = (ulonglong)((unaff_EBX & 0x3ffff) >> 0xb);
      if ((uVar16 < (ulonglong)(*(longlong *)(lVar19 + 0x10) - *(longlong *)(lVar19 + 8) >> 3)) &&
         ((lVar27 = *(longlong *)(*(longlong *)(lVar19 + 8) + uVar16 * 8), lVar27 != 0 &&
          (uVar3 = *(uint *)(lVar27 + (ulonglong)(unaff_EBX & 0x7ff) * 4),
          (unaff_EBX & 0xfffc0000 ^ uVar3) < 0x3ffff)))) {
        lVar27 = (ulonglong)(uVar3 & 0x7f) * 0x250 +
                 *(longlong *)(*(longlong *)(lVar19 + 0x50) + (ulonglong)((uVar3 & 0x3ff80) >> 4));
        goto LAB_141348c45;
      }
    }
  }
  lVar27 = 0;
LAB_141348c45:
  if (lStack_1b0 != 0) {
    LOCK();
    piVar1 = (int *)(lStack_1b0 + 0xc);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
    }
  }
  plVar15 = (longlong *)0x151d325b8;
  if (lVar27 != 0) {
    plVar15 = (longlong *)(lVar27 + 0xc0);
  }
  plVar21 = (longlong *)0x151d325b0;
  if (lVar27 != 0) {
    plVar21 = (longlong *)(lVar27 + 0xb8);
  }
  lVar19 = *plVar21;
  lVar27 = *plVar15;
  lStack_f8 = 0;
  lStack_f0 = 0;
  uStack_e8 = 0;
  lVar20 = *(longlong *)(lStack_b8 + 0x30);
  lStack_100 = *(longlong *)(lStack_b8 + 0x38);
  pcStack_1b8 = (char *)((lStack_100 - lVar20 >> 3) * -0x2c8590b21642c859);
  if (lStack_100 != lVar20) {
    if ((ulonglong)pcStack_1b8 >> 0x3b != 0) {
LAB_14134a1e1:
      FUN_140075e50();
      pcVar10 = (code *)swi(3);
      (*pcVar10)();
      return;
    }
    FUN_14013e140(&lStack_f8,&pcStack_1b8);
    lVar20 = *(longlong *)(lStack_b8 + 0x30);
    lStack_100 = *(longlong *)(lStack_b8 + 0x38);
  }
LAB_141348d24:
  do {
    if (lVar20 == lStack_100) {
      if (lStack_f8 != lStack_f0) {
        puVar18 = (undefined8 *)(*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
        if (puVar18[1] == 0) {
          lVar19 = 0;
        }
        else {
          LOCK();
          piVar1 = (int *)(puVar18[1] + 8);
          *piVar1 = *piVar1 + 1;
          UNLOCK();
          lVar19 = puVar18[1];
        }
        pcStack_1b8 = (char *)*puVar18;
        lStack_1b0 = lVar19;
        if (pcStack_1b8 != (char *)0x0) {
          uVar17 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
          FUN_14059d560(uVar17,&lStack_f8);
        }
        if (lVar19 != 0) {
          LOCK();
          piVar1 = (int *)(lVar19 + 8);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar19);
            LOCK();
            piVar1 = (int *)(lVar19 + 0xc);
            *piVar1 = *piVar1 + -1;
            UNLOCK();
            if (*piVar1 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar19);
            }
          }
        }
      }
      uVar17 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
      if ((*(longlong *)(param_1 + 600) == 0) &&
         (cVar12 = FUN_140289fe0(&UNK_14ec0360b,&UNK_14ec4122d,300,&UNK_14ec489d8,&UNK_14ed61d45),
         cVar12 != '\0')) {
        _DAT_00000000 = 0xdeadc0de;
      }
      if ((**(char **)(param_1 + 600) == '\0') &&
         (cVar12 = FUN_140289fe0(&UNK_14ec89e5e,&UNK_14ec87783,0x130,&UNK_14ec489d8,&UNK_14ed61d45),
         cVar12 != '\0')) {
        _DAT_00000000 = 0xdeadc0de;
      }
      lVar19 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
      lStack_d8 = 0;
      uStack_d0 = 0;
      lStack_c8 = 0;
      if (lVar19 != 0) {
        if (*(longlong *)(lVar19 + 0x10) == 0) {
          lStack_d8 = *(longlong *)(lVar19 + 8);
          uStack_d0 = 0;
        }
        else {
          LOCK();
          piVar1 = (int *)(*(longlong *)(lVar19 + 0x10) + 8);
          *piVar1 = *piVar1 + 1;
          UNLOCK();
          lStack_d8 = *(longlong *)(lVar19 + 8);
          uStack_d0 = *(undefined8 *)(lVar19 + 0x10);
        }
        if (lStack_d8 != 0) {
          lStack_c8 = lVar19;
        }
      }
      (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(uVar17,&lStack_d8);
      *(undefined1 *)(lStack_b8 + 0x11) = 1;
      FUN_140075f00(&lStack_f8);
      return;
    }
    if (*(char *)(lVar20 + 0xb0) == '\0') break;
    if (*(char *)(lVar20 + 0xb0) != '\x01') goto LAB_14134a1dc;
    lVar28 = *(longlong *)(lVar20 + 8);
    lVar6 = *(longlong *)(lVar20 + 0x10);
    lVar26 = lVar20 + 0x88;
    cVar12 = FUN_14134a830(lVar26);
    lVar22 = lVar20 + 8;
    if (cVar12 == '\0') {
      cVar12 = '\0';
    }
    else {
      cVar12 = FUN_14132de40(param_1,lVar22,lVar26);
    }
    if (lVar6 == lVar27 && lVar28 == lVar19) {
      cVar13 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
      ppppppcStack_80 = (char ******)CONCAT44(ppppppcStack_80._4_4_,(int)cVar13);
      ppppppuStack_260 = (undefined8 *******)0x1;
      ppppppcStack_258 = (char ******)&ppppppcStack_80;
      ppppppuStack_b0 = (undefined8 ******)&UNK_14ef4bfdd;
      ppcStack_a8 = (char **)0x33;
      FUN_14e02a400(&pcStack_1b8,&ppppppuStack_b0,&ppppppuStack_260);
      if (uStack_1a0 < 0x10) {
LAB_1413497ba:
        uVar17 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
        FUN_1401e7480(uVar17,lVar20 + 0x20);
        goto LAB_1413497da;
      }
      uVar16 = uStack_1a0 + 1;
      pcVar23 = pcStack_1b8;
      if (uVar16 < 0x1000) {
LAB_1413497b5:
        FUN_14e021350(pcVar23,uVar16);
        goto LAB_1413497ba;
      }
      pcVar23 = *(char **)(pcStack_1b8 + -8);
      if (pcStack_1b8 + (-8 - (longlong)pcVar23) < (char *)0x20) {
        uVar16 = uStack_1a0 + 0x28;
        goto LAB_1413497b5;
      }
      goto LAB_14134a1c3;
    }
    cVar13 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
    ppppppcStack_80 = (char ******)CONCAT44(ppppppcStack_80._4_4_,(int)cVar13);
    ppppppuStack_260 = (undefined8 *******)0x1;
    ppppppcStack_258 = (char ******)&ppppppcStack_80;
    ppppppuStack_b0 = (undefined8 ******)&UNK_14ef4c011;
    ppcStack_a8 = (char **)0x34;
    FUN_14e02a400(&pcStack_1b8,&ppppppuStack_b0,&ppppppuStack_260);
    if (0xf < uStack_1a0) {
      uVar16 = uStack_1a0 + 1;
      pcVar23 = pcStack_1b8;
      if (0xfff < uVar16) {
        pcVar23 = *(char **)(pcStack_1b8 + -8);
        if ((char *)0x1f < pcStack_1b8 + (-8 - (longlong)pcVar23)) goto LAB_14134a1c3;
        uVar16 = uStack_1a0 + 0x28;
      }
      FUN_14e021350(pcVar23,uVar16);
    }
LAB_1413497da:
    FUN_143cbf1f0(lVar20,&pcStack_1b8);
    if ((*(longlong *)(param_1 + 600) == 0) &&
       (cVar13 = FUN_140289fe0(&UNK_14ec0360b,&UNK_14ec4122d,300,&UNK_14ec489d8,&UNK_14ed61d45),
       cVar13 != '\0')) {
      _DAT_00000000 = 0xdeadc0de;
    }
    if ((**(char **)(param_1 + 600) == '\0') &&
       (cVar13 = FUN_140289fe0(&UNK_14ec89e5e,&UNK_14ec87783,0x130,&UNK_14ec489d8,&UNK_14ed61d45),
       cVar13 != '\0')) {
      _DAT_00000000 = 0xdeadc0de;
    }
    (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)
              (*(undefined8 *)(param_1 + 0x268),&ppppppcStack_80);
    if (((char *******)ppppppcStack_80 == (char *******)0x0) &&
       (cVar13 = FUN_140289fe0(&UNK_14ec0360b,&UNK_14ec4122d,300,&UNK_14ec489d8,&UNK_14ed315bc),
       cVar13 != '\0')) {
      _DAT_00000000 = 0xdeadc0de;
    }
    if ((*(char *)ppppppcStack_80 == '\0') &&
       (cVar13 = FUN_140289fe0(&UNK_14ec89e5e,&UNK_14ec87783,0x130,&UNK_14ec489d8,&UNK_14ed315bc),
       cVar13 != '\0')) {
      _DAT_00000000 = 0xdeadc0de;
    }
    uVar16 = uStack_70;
    if (cVar12 == '\0') {
      FUN_142647fe0(&pcStack_1b8,&ppppppuStack_260);
    }
    else {
      FUN_142647bd0(&ppppppuStack_260,&pcStack_1b8);
    }
    FUN_1434e62b0(uVar16,lVar22,&ppppppuStack_260);
    FUN_142647a70(&ppppppuStack_260);
    ppppppuVar11 = ppppppuStack_78;
    ppppppcStack_80 = (char ******)0x0;
    ppppppuStack_78 = (undefined8 ******)0x0;
    if ((undefined8 *******)ppppppuVar11 != (undefined8 *******)0x0) {
      LOCK();
      pppppppuVar25 = (undefined8 *******)(ppppppuVar11 + 1);
      *(int *)pppppppuVar25 = *(int *)pppppppuVar25 + -1;
      UNLOCK();
      if (*(int *)pppppppuVar25 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(ppppppuVar11);
        LOCK();
        piVar1 = (int *)((longlong)ppppppuVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(ppppppuVar11);
        }
      }
      ppppppuVar11 = ppppppuStack_78;
      uStack_70 = 0;
      if (ppppppuStack_78 != (undefined8 ******)0x0) {
        LOCK();
        piVar1 = (int *)((longlong)ppppppuStack_78 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(ppppppuStack_78);
          LOCK();
          piVar1 = (int *)((longlong)ppppppuVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(ppppppuVar11);
          }
        }
      }
    }
    if ((*(longlong *)(param_1 + 600) == 0) &&
       (cVar13 = FUN_140289fe0(&UNK_14ec0360b,&UNK_14ec4122d,300,&UNK_14ec489d8,&UNK_14ed61d45),
       cVar13 != '\0')) {
      _DAT_00000000 = 0xdeadc0de;
    }
    if ((**(char **)(param_1 + 600) == '\0') &&
       (cVar13 = FUN_140289fe0(&UNK_14ec89e5e,&UNK_14ec87783,0x130,&UNK_14ec489d8,&UNK_14ed61d45),
       cVar13 != '\0')) {
      _DAT_00000000 = 0xdeadc0de;
    }
    lVar28 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
    puVar9 = *(undefined **)(lVar28 + 0x10);
    uVar17 = *(undefined8 *)(lVar20 + 8);
    uVar8 = *(undefined8 *)(lVar20 + 0x10);
    uStack_210 = (uint)uVar8 & 0xffff;
    uStack_250 = (undefined8 *******)
                 (CONCAT44(uStack_250._4_4_,(int)((ulonglong)uVar17 >> 0x10)) & 0xffffffff0000ffff);
    uStack_240 = (undefined *)(CONCAT44(uStack_240._4_4_,(int)uVar17) & 0xffffffff0000ffff);
    lStack_230 = CONCAT44(lStack_230._4_4_,(uint)(ushort)((ulonglong)uVar8 >> 0x30));
    uStack_220 = (undefined4)((ulonglong)uVar8 >> 0x10);
    ppppppcStack_80 = (char ******)0x122112;
    ppppppuStack_78 = &ppppppuStack_260;
    puStack_90 = &UNK_14ebaf4a4;
    ppppppuStack_88 = (undefined8 ******)0x28;
    ppppppuStack_260._0_4_ = (int)((ulonglong)uVar17 >> 0x20);
    FUN_14e02a400(&ppppppuStack_b0,&puStack_90,&ppppppcStack_80);
    cVar13 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
    ppppppuStack_88 = &ppppppuStack_260;
    ppppppuStack_260 = (undefined8 ******)CONCAT44(ppppppuStack_260._4_4_,(int)cVar13);
    uStack_250 = &ppppppuStack_b0;
    if (0xf < uStack_98) {
      uStack_250 = (undefined8 *******)ppppppuStack_b0;
    }
    uStack_248 = uStack_a0;
    puStack_90 = (undefined *)0x4d1;
    puStack_110 = &UNK_14ebac711;
    uStack_108 = 0x60;
    uStack_240 = puVar9;
    FUN_14e02a400(&ppppppcStack_80,&puStack_110,&puStack_90);
    if (0xf < uStack_68) {
      uVar16 = uStack_68 + 1;
      pppppppcVar24 = (char *******)ppppppcStack_80;
      if (0xfff < uVar16) {
        pppppppcVar24 = (char *******)ppppppcStack_80[-1];
        if ((char *)0x1f < (char *)((longlong)ppppppcStack_80 + (-8 - (longlong)pppppppcVar24)))
        goto LAB_14134a1c3;
        uVar16 = uStack_68 + 0x28;
      }
      FUN_14e021350(pppppppcVar24,uVar16);
    }
    uStack_70 = 0;
    uStack_68 = 0xf;
    ppppppcStack_80 = (char ******)((ulonglong)ppppppcStack_80 & 0xffffffffffffff00);
    if (0xf < uStack_98) {
      uVar16 = uStack_98 + 1;
      pppppppuVar25 = (undefined8 *******)ppppppuStack_b0;
      if (0xfff < uVar16) {
        pppppppuVar25 = (undefined8 *******)ppppppuStack_b0[-1];
        if ((undefined *)0x1f <
            (undefined *)((longlong)ppppppuStack_b0 + (-8 - (longlong)pppppppuVar25)))
        goto LAB_14134a1c3;
        uVar16 = uStack_98 + 0x28;
      }
      FUN_14e021350(pppppppuVar25,uVar16);
    }
    if ((*(longlong *)(param_1 + 600) == 0) &&
       (cVar13 = FUN_140289fe0(&UNK_14ec0360b,&UNK_14ec4122d,300,&UNK_14ec489d8,&UNK_14ed61d45),
       cVar13 != '\0')) {
      _DAT_00000000 = 0xdeadc0de;
    }
    if ((**(char **)(param_1 + 600) == '\0') &&
       (cVar13 = FUN_140289fe0(&UNK_14ec89e5e,&UNK_14ec87783,0x130,&UNK_14ec489d8,&UNK_14ed61d45),
       cVar13 != '\0')) {
      _DAT_00000000 = 0xdeadc0de;
    }
    lVar28 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(*(undefined8 *)(param_1 + 0x268),lVar22)
    ;
    if (cVar12 == '\x01' && lVar28 != 0) {
      lVar6 = *(longlong *)(lVar20 + 0x88);
      cVar12 = *(char *)(lVar6 + 0x20a);
      uVar17 = *(undefined8 *)(lVar20 + 8);
      uVar8 = *(undefined8 *)(lVar20 + 0x10);
      uStack_210 = (uint)uVar8 & 0xffff;
      ppppppuStack_260 =
           (undefined8 ******)CONCAT44(ppppppuStack_260._4_4_,(int)((ulonglong)uVar17 >> 0x20));
      uStack_250 = (undefined8 *******)
                   (CONCAT44(uStack_250._4_4_,(int)((ulonglong)uVar17 >> 0x10)) & 0xffffffff0000ffff
                   );
      uStack_240 = (undefined *)(CONCAT44(uStack_240._4_4_,(int)uVar17) & 0xffffffff0000ffff);
      lStack_230 = CONCAT44(lStack_230._4_4_,(uint)(ushort)((ulonglong)uVar8 >> 0x30));
      uStack_220 = (undefined4)((ulonglong)uVar8 >> 0x10);
      ppppppcStack_80 = (char ******)0x122112;
      ppppppuStack_78 = &ppppppuStack_260;
      puStack_90 = &UNK_14ebaf4a4;
      ppppppuStack_88 = (undefined8 ******)0x28;
      FUN_14e02a400(&ppppppuStack_b0,&puStack_90,&ppppppcStack_80);
      uStack_240 = &UNK_14ec5ccb6;
      if (cVar12 != '\0') {
        uStack_240 = &UNK_14ec5527d;
      }
      ppppppuStack_260 = &ppppppuStack_b0;
      if (0xf < uStack_98) {
        ppppppuStack_260 = ppppppuStack_b0;
      }
      ppppppuStack_88 = &ppppppuStack_260;
      if (*(ulonglong *)(lVar28 + 0xbd8) < 0x10) {
        uStack_250 = (undefined8 *******)(lVar28 + 0xbc0);
      }
      else {
        uStack_250 = *(undefined8 ********)(lVar28 + 0xbc0);
      }
      uStack_248 = *(ulonglong *)(lVar28 + 0xbd0);
      if (*(ulonglong *)(lVar6 + 0x58) < 0x10) {
        lStack_230 = lVar6 + 0x40;
      }
      else {
        lStack_230 = *(longlong *)(lVar6 + 0x40);
      }
      uStack_228 = *(undefined8 *)(lVar6 + 0x50);
      puStack_90 = (undefined *)0xdcdc;
      puStack_110 = &UNK_14ebae018;
      uStack_108 = 0x67;
      FUN_14e02a400(&ppppppcStack_80,&puStack_110,&puStack_90);
      if (0xf < uStack_68) {
        uVar16 = uStack_68 + 1;
        pppppppcVar24 = (char *******)ppppppcStack_80;
        if (0xfff < uVar16) {
          pppppppcVar24 = (char *******)ppppppcStack_80[-1];
          if ((char *)0x1f < (char *)((longlong)ppppppcStack_80 + (-8 - (longlong)pppppppcVar24)))
          goto LAB_14134a1c3;
          uVar16 = uStack_68 + 0x28;
        }
        FUN_14e021350(pppppppcVar24,uVar16);
      }
      uStack_70 = 0;
      uStack_68 = 0xf;
      ppppppcStack_80 = (char ******)((ulonglong)ppppppcStack_80 & 0xffffffffffffff00);
      if (0xf < uStack_98) {
        uVar16 = uStack_98 + 1;
        pppppppuVar25 = (undefined8 *******)ppppppuStack_b0;
        if (0xfff < uVar16) {
          pppppppuVar25 = (undefined8 *******)ppppppuStack_b0[-1];
          if ((undefined *)0x1f <
              (undefined *)((longlong)ppppppuStack_b0 + (-8 - (longlong)pppppppuVar25)))
          goto LAB_14134a1c3;
          uVar16 = uStack_98 + 0x28;
        }
        FUN_14e021350(pppppppuVar25,uVar16);
      }
      (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(*(undefined8 *)(param_1 + 0x3a0),lVar22,lVar26)
      ;
    }
    FUN_14019dd30(&lStack_f8,lVar20 + 0x40);
    FUN_142647a70(&pcStack_1b8);
    lVar20 = lVar20 + 0xb8;
  } while( true );
  if ((*(longlong *)(param_1 + 600) == 0) &&
     (cVar12 = FUN_140289fe0(&UNK_14ec0360b,&UNK_14ec4122d,300,&UNK_14ec489d8,&UNK_14ed61d45),
     cVar12 != '\0')) {
    _DAT_00000000 = 0xdeadc0de;
  }
  if ((**(char **)(param_1 + 600) == '\0') &&
     (cVar12 = FUN_140289fe0(&UNK_14ec89e5e,&UNK_14ec87783,0x130,&UNK_14ec489d8,&UNK_14ed61d45),
     cVar12 != '\0')) {
    _DAT_00000000 = 0xdeadc0de;
  }
  plVar15 = (longlong *)(*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
  lVar22 = *plVar15;
  if (lVar22 != plVar15[1]) {
    do {
      if (*(char *)(lVar22 + 0x18) == '\x01') {
        lVar26 = *(longlong *)(lVar22 + 8);
        uVar3 = *(uint *)(lVar22 + 0x10);
        lVar28 = *(longlong *)(lVar26 + 0x48);
        iVar14 = (int)((ulonglong)(*(longlong *)(lVar26 + 0x50) - lVar28) >> 3);
        plVar21 = (longlong *)(lVar28 + (ulonglong)(iVar14 - 1U & 0xb845379) * 8);
        lVar6 = *(longlong *)(lVar26 + 0x68);
        do {
          if (*plVar21 == -1) goto LAB_141348e10;
          lVar29 = *plVar21 * 0x20;
          plVar21 = (longlong *)(lVar6 + lVar29);
        } while (*(int *)(lVar6 + 8 + lVar29) != 0xb845379);
        if ((*(longlong *)(lVar26 + 0x70) != lVar29 + lVar6) &&
           (lVar29 = *(longlong *)(lVar29 + lVar6 + 0x10), lVar29 != 0)) {
          uVar16 = (ulonglong)((uVar3 & 0x3ffff) >> 0xb);
          if ((uVar16 < (ulonglong)(*(longlong *)(lVar29 + 0x10) - *(longlong *)(lVar29 + 8) >> 3))
             && (lVar7 = *(longlong *)(*(longlong *)(lVar29 + 8) + uVar16 * 8), lVar7 != 0)) {
            uVar4 = *(uint *)(lVar7 + (ulonglong)(uVar3 & 0x7ff) * 4);
            if ((uVar4 ^ uVar3 & 0xfffc0000) < 0x3ffff) {
              lVar29 = *(longlong *)
                        (*(longlong *)(lVar29 + 0x50) + (ulonglong)((uVar4 & 0x3ff80) >> 4));
              if (lVar29 != 0) {
                plVar21 = (longlong *)(lVar28 + (ulonglong)(iVar14 - 1U & 0x85b93800) * 8);
                do {
                  if (*plVar21 == -1) goto LAB_141348e10;
                  lVar28 = *plVar21 * 0x20;
                  plVar21 = (longlong *)(lVar6 + lVar28);
                } while (*(int *)(lVar6 + 8 + lVar28) != -0x7a46c800);
                if ((((*(longlong *)(lVar26 + 0x70) != lVar6 + lVar28) &&
                     (lVar26 = *(longlong *)(lVar6 + lVar28 + 0x10), lVar26 != 0)) &&
                    (uVar16 < (ulonglong)
                              (*(longlong *)(lVar26 + 0x10) - *(longlong *)(lVar26 + 8) >> 3))) &&
                   (((lVar28 = *(longlong *)(*(longlong *)(lVar26 + 8) + uVar16 * 8), lVar28 != 0 &&
                     (uVar5 = *(uint *)(lVar28 + (ulonglong)(uVar3 & 0x7ff) * 4),
                     (uVar3 & 0xfffc0000 ^ uVar5) < 0x3ffff)) &&
                    ((lVar26 = *(longlong *)
                                (*(longlong *)(lVar26 + 0x50) + (ulonglong)(uVar5 >> 4 & 0x3ff8)),
                     lVar26 != 0 &&
                     ((*(char *)(*(longlong *)(lVar26 + (ulonglong)(uVar5 & 0x7f) * 8) + 0x269) ==
                       '\0' && (lVar26 = (ulonglong)(uVar4 & 0x7f) * 0x250,
                               *(longlong *)(lVar29 + 0xc0 + lVar26) == *(longlong *)(lVar20 + 0x10)
                               && *(longlong *)(lVar20 + 8) == *(longlong *)(lVar29 + 0xb8 + lVar26)
                               )))))))) {
                  cVar12 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
                  ppppppcStack_80 = (char ******)CONCAT44(ppppppcStack_80._4_4_,(int)cVar12);
                  ppppppuStack_260 = (undefined8 *******)0x1;
                  ppppppcStack_258 = (char ******)&ppppppcStack_80;
                  ppppppuStack_b0 = (undefined8 ******)&UNK_14ef6e6b5;
                  ppcStack_a8 = (char **)0x6f;
                  FUN_14e02a400(&pcStack_1b8,&ppppppuStack_b0,&ppppppuStack_260);
                  if (0xf < uStack_1a0) {
                    uVar16 = uStack_1a0 + 1;
                    pcVar23 = pcStack_1b8;
                    if (0xfff < uVar16) {
                      pcVar23 = *(char **)(pcStack_1b8 + -8);
                      if ((char *)0x1f < pcStack_1b8 + (-8 - (longlong)pcVar23)) goto LAB_14134a1c3;
                      uVar16 = uStack_1a0 + 0x28;
                    }
                    FUN_14e021350(pcVar23,uVar16);
                  }
                  if ((*(longlong *)(param_1 + 600) == 0) &&
                     (cVar12 = FUN_140289fe0(&UNK_14ec0360b,&UNK_14ec4122d,300,&UNK_14ec489d8,
                                             &UNK_14ed61d45), cVar12 != '\0')) {
                    _DAT_00000000 = 0xdeadc0de;
                  }
                  if ((**(char **)(param_1 + 600) == '\0') &&
                     (cVar12 = FUN_140289fe0(&UNK_14ec89e5e,&UNK_14ec87783,0x130,&UNK_14ec489d8,
                                             &UNK_14ed61d45), cVar12 != '\0')) {
                    _DAT_00000000 = 0xdeadc0de;
                  }
                  (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)
                            (*(undefined8 *)(param_1 + 0x268),&pcStack_1b8);
                  if ((pcStack_1b8 == (char *)0x0) &&
                     (cVar12 = FUN_140289fe0(&UNK_14ec0360b,&UNK_14ec4122d,300,&UNK_14ec489d8,
                                             &UNK_14ed315bc), cVar12 != '\0')) {
                    _DAT_00000000 = 0xdeadc0de;
                  }
                  if ((*pcStack_1b8 == '\0') &&
                     (cVar12 = FUN_140289fe0(&UNK_14ec89e5e,&UNK_14ec87783,0x130,&UNK_14ec489d8,
                                             &UNK_14ed315bc), cVar12 != '\0')) {
                    _DAT_00000000 = 0xdeadc0de;
                  }
                  FUN_1434e65a0(uStack_1a8,lVar20 + 8);
                  lVar22 = lStack_1b0;
                  pcStack_1b8 = (char *)0x0;
                  lStack_1b0 = 0;
                  if (lVar22 != 0) {
                    LOCK();
                    piVar1 = (int *)(lVar22 + 8);
                    *piVar1 = *piVar1 + -1;
                    UNLOCK();
                    if (*piVar1 == 0) {
                      (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar22);
                      LOCK();
                      piVar1 = (int *)(lVar22 + 0xc);
                      *piVar1 = *piVar1 + -1;
                      UNLOCK();
                      if (*piVar1 == 0) {
                        (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar22);
                      }
                    }
                    lVar22 = lStack_1b0;
                    uStack_1a8 = (char *******)0x0;
                    if (lStack_1b0 != 0) {
                      LOCK();
                      piVar1 = (int *)(lStack_1b0 + 8);
                      *piVar1 = *piVar1 + -1;
                      UNLOCK();
                      if (*piVar1 == 0) {
                        (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lStack_1b0);
                        LOCK();
                        piVar1 = (int *)(lVar22 + 0xc);
                        *piVar1 = *piVar1 + -1;
                        UNLOCK();
                        if (*piVar1 == 0) {
                          (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar22);
                        }
                      }
                    }
                  }
                  lVar20 = lVar20 + 0xb8;
                  goto LAB_141348d24;
                }
              }
            }
          }
        }
      }
LAB_141348e10:
      lVar22 = lVar22 + 0x20;
    } while (lVar22 != plVar15[1]);
  }
  uVar17 = *(undefined8 *)(lVar20 + 8);
  uVar8 = *(undefined8 *)(lVar20 + 0x10);
  uStack_168 = (uint)uVar8 & 0xffff;
  uStack_198 = (uint)uVar17 & 0xffff;
  uStack_1a8 = (char *******)
               (CONCAT44(uStack_1a8._4_4_,(int)((ulonglong)uVar17 >> 0x10)) & 0xffffffff0000ffff);
  uStack_188 = (uint)(ushort)((ulonglong)uVar8 >> 0x30);
  uStack_178 = (undefined4)((ulonglong)uVar8 >> 0x10);
  ppppppuStack_b0 = (undefined8 ******)0x122112;
  ppcStack_a8 = &pcStack_1b8;
  puStack_90 = &UNK_14ebaf4a4;
  ppppppuStack_88 = (undefined8 ******)0x28;
  pcStack_1b8._0_4_ = (int)((ulonglong)uVar17 >> 0x20);
  FUN_14e02a400(&ppppppcStack_80,&puStack_90,&ppppppuStack_b0);
  cVar12 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)();
  pcStack_1b8 = (char *)CONCAT44(pcStack_1b8._4_4_,(int)cVar12);
  uStack_1a8 = &ppppppcStack_80;
  if (0xf < uStack_68) {
    uStack_1a8 = (char *******)ppppppcStack_80;
  }
  uStack_1a0 = uStack_70;
  ppppppuStack_b0 = (undefined8 *******)0xd1;
  ppcStack_a8 = &pcStack_1b8;
  puStack_90 = &UNK_14ebad1a5;
  ppppppuStack_88 = (undefined8 *******)0x24;
  FUN_14e02a400(&ppppppuStack_260,&puStack_90,&ppppppuStack_b0);
  if (0xf < uStack_248) {
    uVar16 = uStack_248 + 1;
    pppppppuVar25 = (undefined8 *******)ppppppuStack_260;
    if (0xfff < uVar16) {
      pppppppuVar25 = (undefined8 *******)ppppppuStack_260[-1];
      if ((undefined *)0x1f <
          (undefined *)((longlong)ppppppuStack_260 + (-8 - (longlong)pppppppuVar25)))
      goto LAB_14134a1c3;
      uVar16 = uStack_248 + 0x28;
    }
    FUN_14e021350(pppppppuVar25,uVar16);
  }
  uStack_250 = (undefined8 *******)0x0;
  uStack_248 = 0xf;
  ppppppuStack_260 = (undefined8 ******)((ulonglong)ppppppuStack_260 & 0xffffffffffffff00);
  if (0xf < uStack_68) {
    uVar16 = uStack_68 + 1;
    pppppppcVar24 = (char *******)ppppppcStack_80;
    if (0xfff < uVar16) {
      pppppppcVar24 = (char *******)ppppppcStack_80[-1];
      if ((char *)0x1f < (char *)((longlong)ppppppcStack_80 + (-8 - (longlong)pppppppcVar24))) {
LAB_14134a1c3:
        _invoke_watson(0,0,0,0,0);
LAB_14134a1dc:
        FUN_1401202c0();
        goto LAB_14134a1e1;
      }
      uVar16 = uStack_68 + 0x28;
    }
    FUN_14e021350(pppppppcVar24,uVar16);
  }
  (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(*(undefined8 *)(param_1 + 0x3a0),lVar20 + 8);
  if ((*(longlong *)(param_1 + 600) == 0) &&
     (cVar12 = FUN_140289fe0(&UNK_14ec0360b,&UNK_14ec4122d,300,&UNK_14ec489d8,&UNK_14ed61d45),
     cVar12 != '\0')) {
    _DAT_00000000 = 0xdeadc0de;
  }
  if ((**(char **)(param_1 + 600) == '\0') &&
     (cVar12 = FUN_140289fe0(&UNK_14ec89e5e,&UNK_14ec87783,0x130,&UNK_14ec489d8,&UNK_14ed61d45),
     cVar12 != '\0')) {
    _DAT_00000000 = 0xdeadc0de;
  }
  (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(*(undefined8 *)(param_1 + 0x268),&pcStack_1b8);
  if ((pcStack_1b8 == (char *)0x0) &&
     (cVar12 = FUN_140289fe0(&UNK_14ec0360b,&UNK_14ec4122d,300,&UNK_14ec489d8,&UNK_14ed315bc),
     cVar12 != '\0')) {
    _DAT_00000000 = 0xdeadc0de;
  }
  if ((*pcStack_1b8 == '\0') &&
     (cVar12 = FUN_140289fe0(&UNK_14ec89e5e,&UNK_14ec87783,0x130,&UNK_14ec489d8,&UNK_14ed315bc),
     cVar12 != '\0')) {
    _DAT_00000000 = 0xdeadc0de;
  }
  FUN_1434e65a0(uStack_1a8,lVar20 + 8);
  lVar22 = lStack_1b0;
  pcStack_1b8 = (char *)0x0;
  lStack_1b0 = 0;
  if (lVar22 != 0) {
    LOCK();
    piVar1 = (int *)(lVar22 + 8);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar22);
      LOCK();
      piVar1 = (int *)(lVar22 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar22);
      }
    }
    lVar22 = lStack_1b0;
    uStack_1a8 = (char *******)0x0;
    if (lStack_1b0 != 0) {
      LOCK();
      piVar1 = (int *)(lStack_1b0 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lStack_1b0);
        LOCK();
        piVar1 = (int *)(lVar22 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar22);
        }
      }
    }
  }
  lVar20 = lVar20 + 0xb8;
  goto LAB_141348d24;
}

