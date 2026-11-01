
/* WARNING: Removing unreachable block (ram,0x000146bde81e) */
/* WARNING: Removing unreachable block (ram,0x000146bde824) */
/* WARNING: Removing unreachable block (ram,0x000146bde842) */
/* WARNING: Removing unreachable block (ram,0x000146bde855) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Restarted to delay deadcode elimination for space: ram */

void FUN_146bdd410(longlong param_1,undefined8 param_2,longlong param_3)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  longlong lVar5;
  undefined *puVar6;
  undefined8 uVar7;
  longlong lVar8;
  code *pcVar9;
  bool bVar10;
  undefined8 ******ppppppuVar11;
  char cVar12;
  char cVar13;
  int iVar14;
  ulonglong uVar15;
  longlong *plVar16;
  undefined8 uVar17;
  longlong lVar18;
  undefined8 *puVar19;
  longlong lVar20;
  longlong *plVar21;
  char *pcVar22;
  char *******pppppppcVar23;
  undefined8 *******pppppppuVar24;
  uint unaff_EBX;
  longlong lVar25;
  longlong lVar26;
  longlong lVar27;
  bool bVar28;
  ulonglong in_stack_fffffffffffffd48;
  undefined8 ******ppppppuStack_2b0;
  char ******ppppppcStack_2a8;
  undefined8 uStack_2a0;
  ulonglong uStack_298;
  undefined8 uStack_290;
  longlong lStack_280;
  undefined8 uStack_278;
  undefined4 uStack_270;
  uint uStack_260;
  longlong lStack_1e8;
  longlong lStack_1e0;
  char *pcStack_1d8;
  longlong lStack_1d0;
  undefined8 uStack_1c8;
  ulonglong uStack_1c0;
  uint uStack_1b8;
  uint uStack_1a8;
  undefined4 uStack_198;
  uint uStack_188;
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
  if (*(char **)(param_1 + 0x2b0) == (char *)0x0) {
    return;
  }
  if (**(char **)(param_1 + 0x2b0) != '\x01') {
    return;
  }
  lStack_b8 = param_3;
  cVar12 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
  if (cVar12 != '\0') {
    return;
  }
  (*(code *)PTR__guard_dispatch_icall_14f99ef70)(*(undefined8 *)(param_1 + 0x58),&pcStack_1d8);
  lVar27 = lStack_1d0;
  lVar25 = 0;
  if ((lStack_1d0 == 0) || (((uint)uStack_1c8 & 0x3ffff) == 0x3ffff)) goto LAB_146bdd660;
  iVar14 = *(int *)(lStack_1d0 + 8);
  lVar25 = 0;
  do {
    if (iVar14 == 0) goto LAB_146bdd660;
    LOCK();
    iVar2 = *(int *)(lStack_1d0 + 8);
    bVar28 = iVar14 == iVar2;
    if (bVar28) {
      *(int *)(lStack_1d0 + 8) = iVar14 + 1;
      iVar2 = iVar14;
    }
    iVar14 = iVar2;
    UNLOCK();
  } while (!bVar28);
  if (pcStack_1d8 == (char *)0x0) {
LAB_146bdd55b:
    bVar28 = false;
    pcVar22 = pcStack_1d8;
  }
  else {
    uVar15 = (ulonglong)(((uint)uStack_1c8 & 0x3ffff) >> 0xb);
    unaff_EBX = (uint)uStack_1c8;
    if (((((ulonglong)(*(longlong *)(pcStack_1d8 + 0x118) - *(longlong *)(pcStack_1d8 + 0x110) >> 3)
           <= uVar15) ||
         (lVar25 = *(longlong *)(*(longlong *)(pcStack_1d8 + 0x110) + uVar15 * 8), lVar25 == 0)) ||
        (uVar3 = *(uint *)(lVar25 + (ulonglong)((uint)uStack_1c8 & 0x7ff) * 4),
        0x3fffe < ((uint)uStack_1c8 & 0xfffc0000 ^ uVar3))) ||
       (*(ulonglong *)(pcStack_1d8 + 0x150) <= (ulonglong)(uVar3 & 0x3ffff))) goto LAB_146bdd55b;
    bVar28 = true;
    pcVar22 = pcStack_1d8 + 0x30;
  }
  if (lStack_1d0 != 0) {
    LOCK();
    piVar1 = (int *)(lStack_1d0 + 8);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_1d0);
      LOCK();
      piVar1 = (int *)(lVar27 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar27);
      }
    }
  }
  if (bVar28) {
    plVar16 = (longlong *)
              (*(longlong *)(pcVar22 + 0x48) +
              (ulonglong)
              (((uint)(*(int *)(pcVar22 + 0x50) - (int)*(longlong *)(pcVar22 + 0x48)) >> 3) - 1 &
              0xb845379) * 8);
    lVar27 = *(longlong *)(pcVar22 + 0x68);
    lVar25 = 0;
    do {
      if (*plVar16 == -1) goto LAB_146bdd660;
      lVar20 = *plVar16 * 0x20;
      plVar16 = (longlong *)(lVar27 + lVar20);
    } while (*(int *)(lVar27 + 8 + lVar20) != 0xb845379);
    if ((*(longlong *)(pcVar22 + 0x70) != lVar27 + lVar20) &&
       (lVar27 = *(longlong *)(lVar27 + lVar20 + 0x10), lVar27 != 0)) {
      uVar15 = (ulonglong)((unaff_EBX & 0x3ffff) >> 0xb);
      if ((uVar15 < (ulonglong)(*(longlong *)(lVar27 + 0x10) - *(longlong *)(lVar27 + 8) >> 3)) &&
         ((lVar25 = *(longlong *)(*(longlong *)(lVar27 + 8) + uVar15 * 8), lVar25 != 0 &&
          (uVar3 = *(uint *)(lVar25 + (ulonglong)(unaff_EBX & 0x7ff) * 4),
          (unaff_EBX & 0xfffc0000 ^ uVar3) < 0x3ffff)))) {
        lVar25 = (ulonglong)(uVar3 & 0x7f) * 0x250 +
                 *(longlong *)(*(longlong *)(lVar27 + 0x50) + (ulonglong)((uVar3 & 0x3ff80) >> 4));
        goto LAB_146bdd660;
      }
    }
    lVar25 = 0;
  }
  else {
    lVar25 = 0;
  }
LAB_146bdd660:
  if (lStack_1d0 != 0) {
    LOCK();
    piVar1 = (int *)(lStack_1d0 + 0xc);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    }
  }
  plVar16 = (longlong *)0x1528d5fb8;
  if (lVar25 != 0) {
    plVar16 = (longlong *)(lVar25 + 0xc0);
  }
  plVar21 = (longlong *)0x1528d5fb0;
  if (lVar25 != 0) {
    plVar21 = (longlong *)(lVar25 + 0xb8);
  }
  lStack_1e8 = *plVar21;
  lStack_1e0 = *plVar16;
  lStack_f8 = 0;
  lStack_f0 = 0;
  uStack_e8 = 0;
  lVar27 = *(longlong *)(lStack_b8 + 0x30);
  lStack_100 = *(longlong *)(lStack_b8 + 0x38);
  pcStack_1d8 = (char *)((lStack_100 - lVar27 >> 3) * -0x7b425ed097b425ed);
  if (lStack_100 != lVar27) {
    if ((ulonglong)pcStack_1d8 >> 0x3b != 0) {
LAB_146bde8ed:
      FUN_140075b60();
      pcVar9 = (code *)swi(3);
      (*pcVar9)();
      return;
    }
    FUN_14026bb30(&lStack_f8,&pcStack_1d8);
    lVar27 = *(longlong *)(lStack_b8 + 0x30);
    lStack_100 = *(longlong *)(lStack_b8 + 0x38);
  }
LAB_146bdd747:
  do {
    if (lVar27 == lStack_100) {
      if (lStack_f8 != lStack_f0) {
        puVar19 = (undefined8 *)(*(code *)PTR__guard_dispatch_icall_14f99ef70)();
        if (puVar19[1] == 0) {
          lVar27 = 0;
        }
        else {
          LOCK();
          piVar1 = (int *)(puVar19[1] + 8);
          *piVar1 = *piVar1 + 1;
          UNLOCK();
          lVar27 = puVar19[1];
        }
        pcStack_1d8 = (char *)*puVar19;
        lStack_1d0 = lVar27;
        if (pcStack_1d8 != (char *)0x0) {
          uVar17 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
          FUN_140624b50(uVar17,&lStack_f8);
        }
        if (lVar27 != 0) {
          LOCK();
          piVar1 = (int *)(lVar27 + 8);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar27);
            LOCK();
            piVar1 = (int *)(lVar27 + 0xc);
            *piVar1 = *piVar1 + -1;
            UNLOCK();
            if (*piVar1 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar27);
            }
          }
        }
      }
      uVar17 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
      if (*(char **)(param_1 + 0x2b0) == (char *)0x0) {
        _DAT_00000000 = 0xdeadc0de;
      }
      if (**(char **)(param_1 + 0x2b0) == '\0') {
        _DAT_00000000 = 0xdeadc0de;
      }
      lVar27 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
      lStack_d8 = 0;
      uStack_d0 = 0;
      lStack_c8 = 0;
      if (lVar27 != 0) {
        if (*(longlong *)(lVar27 + 0x10) == 0) {
          lVar25 = *(longlong *)(lVar27 + 8);
          uStack_d0 = 0;
          lStack_d8 = lVar25;
        }
        else {
          LOCK();
          piVar1 = (int *)(*(longlong *)(lVar27 + 0x10) + 8);
          *piVar1 = *piVar1 + 1;
          UNLOCK();
          uStack_d0 = *(undefined8 *)(lVar27 + 0x10);
          lVar25 = *(longlong *)(lVar27 + 8);
          lStack_d8 = *(longlong *)(lVar27 + 8);
        }
        if (lVar25 != 0) {
          lStack_c8 = lVar27;
        }
      }
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uVar17,&lStack_d8);
      *(undefined1 *)(lStack_b8 + 0x11) = 1;
      FUN_140075c10(&lStack_f8);
      return;
    }
    if (*(char *)(lVar27 + 0xd0) == '\0') {
      if (*(char **)(param_1 + 0x2b0) == (char *)0x0) {
        _DAT_00000000 = 0xdeadc0de;
      }
      else if (**(char **)(param_1 + 0x2b0) == '\0') {
        _DAT_00000000 = 0xdeadc0de;
      }
      plVar16 = (longlong *)(*(code *)PTR__guard_dispatch_icall_14f99ef70)();
      for (lVar25 = *plVar16; lVar25 != plVar16[1]; lVar25 = lVar25 + 0x20) {
        if (*(char *)(lVar25 + 0x18) == '\x01') {
          lVar20 = *(longlong *)(lVar25 + 8);
          uVar3 = *(uint *)(lVar25 + 0x10);
          lVar18 = *(longlong *)(lVar20 + 0x48);
          iVar14 = (int)((ulonglong)(*(longlong *)(lVar20 + 0x50) - lVar18) >> 3);
          plVar21 = (longlong *)(lVar18 + (ulonglong)(iVar14 - 1U & 0xb845379) * 8);
          lVar8 = *(longlong *)(lVar20 + 0x68);
          do {
            if (*plVar21 == -1) goto LAB_146bdd8c0;
            lVar26 = *plVar21 * 0x20;
            plVar21 = (longlong *)(lVar8 + lVar26);
          } while (*(int *)(lVar8 + 8 + lVar26) != 0xb845379);
          if ((*(longlong *)(lVar20 + 0x70) != lVar26 + lVar8) &&
             (lVar26 = *(longlong *)(lVar26 + lVar8 + 0x10), lVar26 != 0)) {
            uVar15 = (ulonglong)((uVar3 & 0x3ffff) >> 0xb);
            if ((uVar15 < (ulonglong)(*(longlong *)(lVar26 + 0x10) - *(longlong *)(lVar26 + 8) >> 3)
                ) && (lVar5 = *(longlong *)(*(longlong *)(lVar26 + 8) + uVar15 * 8), lVar5 != 0)) {
              uVar4 = *(uint *)(lVar5 + (ulonglong)(uVar3 & 0x7ff) * 4);
              if (((uVar4 ^ uVar3 & 0xfffc0000) < 0x3ffff) &&
                 (lVar26 = *(longlong *)
                            (*(longlong *)(lVar26 + 0x50) + (ulonglong)((uVar4 & 0x3ff80) >> 4)),
                 lVar26 != 0)) {
                lVar26 = lVar26 + (ulonglong)(uVar4 & 0x7f) * 0x250;
                plVar21 = (longlong *)(lVar18 + (ulonglong)(iVar14 - 1U & 0x85b93800) * 8);
                do {
                  if (*plVar21 == -1) goto LAB_146bdd8c0;
                  lVar18 = *plVar21 * 0x20;
                  plVar21 = (longlong *)(lVar8 + lVar18);
                } while (*(int *)(lVar8 + 8 + lVar18) != -0x7a46c800);
                if ((((*(longlong *)(lVar20 + 0x70) != lVar8 + lVar18) &&
                     (lVar20 = *(longlong *)(lVar8 + lVar18 + 0x10), lVar20 != 0)) &&
                    (uVar15 < (ulonglong)
                              (*(longlong *)(lVar20 + 0x10) - *(longlong *)(lVar20 + 8) >> 3))) &&
                   (((lVar18 = *(longlong *)(*(longlong *)(lVar20 + 8) + uVar15 * 8), lVar18 != 0 &&
                     (uVar4 = *(uint *)(lVar18 + (ulonglong)(uVar3 & 0x7ff) * 4),
                     (uVar3 & 0xfffc0000 ^ uVar4) < 0x3ffff)) &&
                    ((lVar20 = *(longlong *)
                                (*(longlong *)(lVar20 + 0x50) + (ulonglong)(uVar4 >> 4 & 0x3ff8)),
                     lVar20 != 0 &&
                     ((*(char *)(*(longlong *)(lVar20 + (ulonglong)(uVar4 & 0x7f) * 8) + 0x289) ==
                       '\0' && (*(longlong *)(lVar26 + 0xc0) == *(longlong *)(lVar27 + 0x10) &&
                                *(longlong *)(lVar27 + 8) == *(longlong *)(lVar26 + 0xb8))))))))) {
                  cVar12 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
                  ppppppcStack_80 = (char ******)CONCAT44(ppppppcStack_80._4_4_,(int)cVar12);
                  ppppppuStack_2b0 = (undefined8 *******)0x1;
                  ppppppcStack_2a8 = (char ******)&ppppppcStack_80;
                  ppppppuStack_b0 = (undefined8 ******)&UNK_14f80a74d;
                  ppcStack_a8 = (char **)0x6f;
                  FUN_14e93d4f0(&pcStack_1d8,&ppppppuStack_b0,&ppppppuStack_2b0);
                  if (0xf < uStack_1c0) {
                    uVar15 = uStack_1c0 + 1;
                    pcVar22 = pcStack_1d8;
                    if (0xfff < uVar15) {
                      pcVar22 = *(char **)(pcStack_1d8 + -8);
                      if ((char *)0x1f < pcStack_1d8 + (-8 - (longlong)pcVar22)) goto LAB_146bde8cd;
                      uVar15 = uStack_1c0 + 0x28;
                    }
                    FUN_14e934b00(pcVar22,uVar15);
                  }
                  if (*(char **)(param_1 + 0x2b0) == (char *)0x0) {
                    _DAT_00000000 = 0xdeadc0de;
                  }
                  if (**(char **)(param_1 + 0x2b0) == '\0') {
                    _DAT_00000000 = 0xdeadc0de;
                  }
                  (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                            (*(undefined8 *)(param_1 + 0x2c0),&pcStack_1d8);
                  if (pcStack_1d8 == (char *)0x0) {
                    _DAT_00000000 = 0xdeadc0de;
                  }
                  if (*pcStack_1d8 == '\0') {
                    _DAT_00000000 = 0xdeadc0de;
                  }
                  FUN_141ee31c0(uStack_1c8,lVar27 + 8);
                  lVar25 = lStack_1d0;
                  pcStack_1d8 = (char *)0x0;
                  lStack_1d0 = 0;
                  if (lVar25 != 0) {
                    LOCK();
                    piVar1 = (int *)(lVar25 + 8);
                    *piVar1 = *piVar1 + -1;
                    UNLOCK();
                    if (*piVar1 == 0) {
                      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar25);
                      LOCK();
                      piVar1 = (int *)(lVar25 + 0xc);
                      *piVar1 = *piVar1 + -1;
                      UNLOCK();
                      if (*piVar1 == 0) {
                        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar25);
                      }
                    }
                    lVar25 = lStack_1d0;
                    uStack_1c8 = (char *******)0x0;
                    if (lStack_1d0 != 0) {
                      LOCK();
                      piVar1 = (int *)(lStack_1d0 + 8);
                      *piVar1 = *piVar1 + -1;
                      UNLOCK();
                      if (*piVar1 == 0) {
                        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_1d0);
                        LOCK();
                        piVar1 = (int *)(lVar25 + 0xc);
                        *piVar1 = *piVar1 + -1;
                        UNLOCK();
                        if (*piVar1 == 0) {
                          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar25);
                        }
                      }
                    }
                  }
                  lVar27 = lVar27 + 0xd8;
                  goto LAB_146bdd747;
                }
              }
            }
          }
        }
LAB_146bdd8c0:
      }
      uVar17 = *(undefined8 *)(lVar27 + 8);
      uVar7 = *(undefined8 *)(lVar27 + 0x10);
      uStack_188 = (uint)uVar7 & 0xffff;
      uStack_1b8 = (uint)uVar17 & 0xffff;
      uStack_1c8 = (char *******)
                   (CONCAT44(uStack_1c8._4_4_,(int)((ulonglong)uVar17 >> 0x10)) & 0xffffffff0000ffff
                   );
      uStack_1a8 = (uint)(ushort)((ulonglong)uVar7 >> 0x30);
      uStack_198 = (undefined4)((ulonglong)uVar7 >> 0x10);
      ppppppuStack_b0 = (undefined8 ******)0x122112;
      puStack_90 = &UNK_14f51112b;
      ppppppuStack_88 = (undefined8 ******)0x28;
      ppcStack_a8 = &pcStack_1d8;
      pcStack_1d8._0_4_ = (int)((ulonglong)uVar17 >> 0x20);
      FUN_14e93d4f0(&ppppppcStack_80,&puStack_90,&ppppppuStack_b0);
      cVar12 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
      pcStack_1d8 = (char *)CONCAT44(pcStack_1d8._4_4_,(int)cVar12);
      uStack_1c8 = &ppppppcStack_80;
      if (0xf < uStack_68) {
        uStack_1c8 = (char *******)ppppppcStack_80;
      }
      uStack_1c0 = uStack_70;
      ppppppuStack_b0 = (undefined8 *******)0xd1;
      puStack_90 = &UNK_14f50efc7;
      ppppppuStack_88 = (undefined8 *******)0x24;
      ppcStack_a8 = &pcStack_1d8;
      FUN_14e93d4f0(&ppppppuStack_2b0,&puStack_90,&ppppppuStack_b0);
      if (0xf < uStack_298) {
        uVar15 = uStack_298 + 1;
        pppppppuVar24 = (undefined8 *******)ppppppuStack_2b0;
        if (uVar15 < 0x1000) {
LAB_146bdddb1:
          FUN_14e934b00(pppppppuVar24,uVar15);
          goto LAB_146bdddb6;
        }
        pppppppuVar24 = (undefined8 *******)ppppppuStack_2b0[-1];
        if ((undefined *)((longlong)ppppppuStack_2b0 + (-8 - (longlong)pppppppuVar24)) <
            (undefined *)0x20) {
          uVar15 = uStack_298 + 0x28;
          goto LAB_146bdddb1;
        }
LAB_146bde8cd:
        _invoke_watson(0,0,0,0,0);
LAB_146bde8e7:
        FUN_1401166e0();
        goto LAB_146bde8ed;
      }
LAB_146bdddb6:
      uStack_2a0 = (undefined8 *******)0x0;
      uStack_298 = 0xf;
      ppppppuStack_2b0 = (undefined8 ******)((ulonglong)ppppppuStack_2b0 & 0xffffffffffffff00);
      if (0xf < uStack_68) {
        uVar15 = uStack_68 + 1;
        pppppppcVar23 = (char *******)ppppppcStack_80;
        if (0xfff < uVar15) {
          pppppppcVar23 = (char *******)ppppppcStack_80[-1];
          if ((char *)0x1f < (char *)((longlong)ppppppcStack_80 + (-8 - (longlong)pppppppcVar23)))
          goto LAB_146bde8cd;
          uVar15 = uStack_68 + 0x28;
        }
        FUN_14e934b00(pppppppcVar23,uVar15);
      }
      lVar25 = lVar27 + 8;
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(*(undefined8 *)(param_1 + 0x3f8),lVar25);
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(*(undefined8 *)(param_1 + 0x3f8),lVar25);
      if (*(char **)(param_1 + 0x2b0) == (char *)0x0) {
        _DAT_00000000 = 0xdeadc0de;
      }
      else if (**(char **)(param_1 + 0x2b0) == '\0') {
        _DAT_00000000 = 0xdeadc0de;
      }
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(*(undefined8 *)(param_1 + 0x2c0),&pcStack_1d8);
      if (pcStack_1d8 == (char *)0x0) {
        _DAT_00000000 = 0xdeadc0de;
      }
      else if (*pcStack_1d8 == '\0') {
        _DAT_00000000 = 0xdeadc0de;
      }
      FUN_141ee31c0(uStack_1c8,lVar25);
      lVar25 = lStack_1d0;
      pcStack_1d8 = (char *)0x0;
      lStack_1d0 = 0;
      if (lVar25 != 0) {
        LOCK();
        piVar1 = (int *)(lVar25 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar25);
          LOCK();
          piVar1 = (int *)(lVar25 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar25);
          }
        }
        lVar25 = lStack_1d0;
        uStack_1c8 = (char *******)0x0;
        if (lStack_1d0 != 0) {
          LOCK();
          piVar1 = (int *)(lStack_1d0 + 8);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_1d0);
            LOCK();
            piVar1 = (int *)(lVar25 + 0xc);
            *piVar1 = *piVar1 + -1;
            UNLOCK();
            if (*piVar1 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar25);
            }
          }
        }
      }
      lVar27 = lVar27 + 0xd8;
      goto LAB_146bdd747;
    }
    if (*(char *)(lVar27 + 0xd0) != '\x01') goto LAB_146bde8e7;
    bVar28 = *(longlong *)(lVar27 + 0x10) != lStack_1e0;
    bVar10 = *(longlong *)(lVar27 + 8) != lStack_1e8;
    lVar20 = lVar27 + 0xa8;
    cVar12 = FUN_146bdee80(lVar20);
    lVar25 = lVar27 + 8;
    if (cVar12 == '\0') {
      cVar13 = '\0';
      cVar12 = '\0';
      if (!bVar28 && !bVar10) goto LAB_146bddf5c;
LAB_146bdd7bb:
      cVar12 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
      ppppppcStack_80 = (char ******)CONCAT44(ppppppcStack_80._4_4_,(int)cVar12);
      ppppppuStack_2b0 = (undefined8 *******)0x1;
      ppppppcStack_2a8 = (char ******)&ppppppcStack_80;
      ppppppuStack_b0 = (undefined8 ******)&UNK_14f7e8306;
      ppcStack_a8 = (char **)0x34;
      FUN_14e93d4f0(&pcStack_1d8,&ppppppuStack_b0,&ppppppuStack_2b0);
      if (0xf < uStack_1c0) {
        uVar15 = uStack_1c0 + 1;
        pcVar22 = pcStack_1d8;
        if (0xfff < uVar15) {
          pcVar22 = *(char **)(pcStack_1d8 + -8);
          if ((char *)0x1f < pcStack_1d8 + (-8 - (longlong)pcVar22)) goto LAB_146bde8cd;
          uVar15 = uStack_1c0 + 0x28;
        }
        FUN_14e934b00(pcVar22,uVar15);
      }
    }
    else {
      cVar13 = FUN_146bc4800(param_1,lVar25,lVar20);
      cVar12 = cVar13;
      if (bVar28 || bVar10) goto LAB_146bdd7bb;
LAB_146bddf5c:
      cVar13 = cVar12;
      cVar12 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
      ppppppcStack_80 = (char ******)CONCAT44(ppppppcStack_80._4_4_,(int)cVar12);
      ppppppuStack_2b0 = (undefined8 *******)0x1;
      ppppppcStack_2a8 = (char ******)&ppppppcStack_80;
      ppppppuStack_b0 = (undefined8 ******)&UNK_14f7e82d2;
      ppcStack_a8 = (char **)0x33;
      FUN_14e93d4f0(&pcStack_1d8,&ppppppuStack_b0,&ppppppuStack_2b0);
      if (0xf < uStack_1c0) {
        uVar15 = uStack_1c0 + 1;
        pcVar22 = pcStack_1d8;
        if (0xfff < uVar15) {
          pcVar22 = *(char **)(pcStack_1d8 + -8);
          if ((char *)0x1f < pcStack_1d8 + (-8 - (longlong)pcVar22)) goto LAB_146bde8cd;
          uVar15 = uStack_1c0 + 0x28;
        }
        FUN_14e934b00(pcVar22,uVar15);
      }
      uVar17 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
      FUN_1402bf940(uVar17,lVar27 + 0x20);
    }
    FUN_144045d50(lVar27,&pcStack_1d8);
    if (*(char **)(param_1 + 0x2b0) == (char *)0x0) {
      _DAT_00000000 = 0xdeadc0de;
    }
    else if (**(char **)(param_1 + 0x2b0) == '\0') {
      _DAT_00000000 = 0xdeadc0de;
    }
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)
              (*(undefined8 *)(param_1 + 0x2c0),&ppppppcStack_80);
    uVar15 = uStack_70;
    if ((char *******)ppppppcStack_80 == (char *******)0x0) {
      _DAT_00000000 = 0xdeadc0de;
    }
    else if (*(char *)ppppppcStack_80 == '\0') {
      _DAT_00000000 = 0xdeadc0de;
    }
    if (cVar13 == '\0') {
      FUN_1402f2c20(&pcStack_1d8,&ppppppuStack_2b0);
    }
    else {
      FUN_1402f26a0(&ppppppuStack_2b0,&pcStack_1d8);
    }
    FUN_141ee2e30(uVar15,lVar25,&ppppppuStack_2b0);
    FUN_1402f1fb0(&ppppppuStack_2b0);
    ppppppuVar11 = ppppppuStack_78;
    ppppppcStack_80 = (char ******)0x0;
    ppppppuStack_78 = (undefined8 ******)0x0;
    if ((undefined8 *******)ppppppuVar11 != (undefined8 *******)0x0) {
      LOCK();
      pppppppuVar24 = (undefined8 *******)(ppppppuVar11 + 1);
      *(int *)pppppppuVar24 = *(int *)pppppppuVar24 + -1;
      UNLOCK();
      if (*(int *)pppppppuVar24 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(ppppppuVar11);
        LOCK();
        piVar1 = (int *)((longlong)ppppppuVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(ppppppuVar11);
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
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(ppppppuStack_78);
          LOCK();
          piVar1 = (int *)((longlong)ppppppuVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(ppppppuVar11);
          }
        }
      }
    }
    if (*(char **)(param_1 + 0x2b0) == (char *)0x0) {
      _DAT_00000000 = 0xdeadc0de;
    }
    else if (**(char **)(param_1 + 0x2b0) == '\0') {
      _DAT_00000000 = 0xdeadc0de;
    }
    lVar18 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    puVar6 = *(undefined **)(lVar18 + 0x10);
    uVar17 = *(undefined8 *)(lVar27 + 8);
    uVar7 = *(undefined8 *)(lVar27 + 0x10);
    uStack_260 = (uint)uVar7 & 0xffff;
    uStack_2a0 = (undefined8 *******)
                 (CONCAT44(uStack_2a0._4_4_,(int)((ulonglong)uVar17 >> 0x10)) & 0xffffffff0000ffff);
    uStack_290 = (undefined *)(CONCAT44(uStack_290._4_4_,(int)uVar17) & 0xffffffff0000ffff);
    lStack_280 = CONCAT44(lStack_280._4_4_,(uint)(ushort)((ulonglong)uVar7 >> 0x30));
    uStack_270 = (undefined4)((ulonglong)uVar7 >> 0x10);
    ppppppcStack_80 = (char ******)0x122112;
    ppppppuStack_78 = &ppppppuStack_2b0;
    puStack_90 = &UNK_14f51112b;
    ppppppuStack_88 = (undefined8 ******)0x28;
    ppppppuStack_2b0._0_4_ = (int)((ulonglong)uVar17 >> 0x20);
    FUN_14e93d4f0(&ppppppuStack_b0,&puStack_90,&ppppppcStack_80);
    cVar12 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    ppppppuStack_2b0 = (undefined8 ******)CONCAT44(ppppppuStack_2b0._4_4_,(int)cVar12);
    uStack_2a0 = &ppppppuStack_b0;
    if (0xf < uStack_98) {
      uStack_2a0 = (undefined8 *******)ppppppuStack_b0;
    }
    uStack_298 = uStack_a0;
    puStack_90 = (undefined *)0x4d1;
    ppppppuStack_88 = &ppppppuStack_2b0;
    puStack_110 = &UNK_14f50e6db;
    uStack_108 = 0x60;
    uStack_290 = puVar6;
    FUN_14e93d4f0(&ppppppcStack_80,&puStack_110,&puStack_90);
    if (0xf < uStack_68) {
      uVar15 = uStack_68 + 1;
      pppppppcVar23 = (char *******)ppppppcStack_80;
      if (0xfff < uVar15) {
        pppppppcVar23 = (char *******)ppppppcStack_80[-1];
        if ((char *)0x1f < (char *)((longlong)ppppppcStack_80 + (-8 - (longlong)pppppppcVar23)))
        goto LAB_146bde8cd;
        uVar15 = uStack_68 + 0x28;
      }
      FUN_14e934b00(pppppppcVar23,uVar15);
    }
    uStack_70 = 0;
    uStack_68 = 0xf;
    ppppppcStack_80 = (char ******)((ulonglong)ppppppcStack_80 & 0xffffffffffffff00);
    if (0xf < uStack_98) {
      uVar15 = uStack_98 + 1;
      pppppppuVar24 = (undefined8 *******)ppppppuStack_b0;
      if (0xfff < uVar15) {
        pppppppuVar24 = (undefined8 *******)ppppppuStack_b0[-1];
        if ((undefined *)0x1f <
            (undefined *)((longlong)ppppppuStack_b0 + (-8 - (longlong)pppppppuVar24)))
        goto LAB_146bde8cd;
        uVar15 = uStack_98 + 0x28;
      }
      FUN_14e934b00(pppppppuVar24,uVar15);
    }
    if (*(char **)(param_1 + 0x2b0) == (char *)0x0) {
      _DAT_00000000 = 0xdeadc0de;
    }
    else if (**(char **)(param_1 + 0x2b0) == '\0') {
      _DAT_00000000 = 0xdeadc0de;
    }
    lVar18 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(*(undefined8 *)(param_1 + 0x2c0),lVar25)
    ;
    if (cVar13 == '\x01' && lVar18 != 0) {
      lVar8 = *(longlong *)(lVar27 + 0xa8);
      cVar12 = *(char *)(lVar8 + 0x1ea);
      uVar17 = *(undefined8 *)(lVar27 + 8);
      uVar7 = *(undefined8 *)(lVar27 + 0x10);
      uStack_260 = (uint)uVar7 & 0xffff;
      ppppppuStack_2b0 =
           (undefined8 ******)CONCAT44(ppppppuStack_2b0._4_4_,(int)((ulonglong)uVar17 >> 0x20));
      uStack_2a0 = (undefined8 *******)
                   (CONCAT44(uStack_2a0._4_4_,(int)((ulonglong)uVar17 >> 0x10)) & 0xffffffff0000ffff
                   );
      uStack_290 = (undefined *)(CONCAT44(uStack_290._4_4_,(int)uVar17) & 0xffffffff0000ffff);
      lStack_280 = CONCAT44(lStack_280._4_4_,(uint)(ushort)((ulonglong)uVar7 >> 0x30));
      uStack_270 = (undefined4)((ulonglong)uVar7 >> 0x10);
      ppppppcStack_80 = (char ******)0x122112;
      ppppppuStack_78 = &ppppppuStack_2b0;
      puStack_90 = &UNK_14f51112b;
      ppppppuStack_88 = (undefined8 ******)0x28;
      FUN_14e93d4f0(&ppppppuStack_b0,&puStack_90,&ppppppcStack_80);
      uStack_290 = &UNK_14f5c4a03;
      if (cVar12 != '\0') {
        uStack_290 = &UNK_14f5bce38;
      }
      ppppppuStack_2b0 = &ppppppuStack_b0;
      if (0xf < uStack_98) {
        ppppppuStack_2b0 = ppppppuStack_b0;
      }
      if (*(ulonglong *)(lVar18 + 0xc00) < 0x10) {
        uStack_2a0 = (undefined8 *******)(lVar18 + 0xbe8);
      }
      else {
        uStack_2a0 = *(undefined8 ********)(lVar18 + 0xbe8);
      }
      uStack_298 = *(ulonglong *)(lVar18 + 0xbf8);
      if (*(ulonglong *)(lVar8 + 0x38) < 0x10) {
        lStack_280 = lVar8 + 0x20;
      }
      else {
        lStack_280 = *(longlong *)(lVar8 + 0x20);
      }
      ppppppuStack_88 = &ppppppuStack_2b0;
      uStack_278 = *(undefined8 *)(lVar8 + 0x30);
      puStack_90 = (undefined *)0xdcdc;
      puStack_110 = &UNK_14f50fc92;
      uStack_108 = 0x67;
      FUN_14e93d4f0(&ppppppcStack_80,&puStack_110,&puStack_90);
      if (0xf < uStack_68) {
        uVar15 = uStack_68 + 1;
        pppppppcVar23 = (char *******)ppppppcStack_80;
        if (0xfff < uVar15) {
          pppppppcVar23 = (char *******)ppppppcStack_80[-1];
          if ((char *)0x1f < (char *)((longlong)ppppppcStack_80 + (-8 - (longlong)pppppppcVar23)))
          goto LAB_146bde8cd;
          uVar15 = uStack_68 + 0x28;
        }
        FUN_14e934b00(pppppppcVar23,uVar15);
      }
      uStack_70 = 0;
      uStack_68 = 0xf;
      ppppppcStack_80 = (char ******)((ulonglong)ppppppcStack_80 & 0xffffffffffffff00);
      if (0xf < uStack_98) {
        uVar15 = uStack_98 + 1;
        pppppppuVar24 = (undefined8 *******)ppppppuStack_b0;
        if (0xfff < uVar15) {
          pppppppuVar24 = (undefined8 *******)ppppppuStack_b0[-1];
          if ((undefined *)0x1f <
              (undefined *)((longlong)ppppppuStack_b0 + (-8 - (longlong)pppppppuVar24)))
          goto LAB_146bde8cd;
          uVar15 = uStack_98 + 0x28;
        }
        FUN_14e934b00(pppppppuVar24,uVar15);
      }
      in_stack_fffffffffffffd48 = in_stack_fffffffffffffd48 & 0xffffffff00000000;
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                (*(undefined8 *)(param_1 + 0x3f8),lVar25,lVar20,lVar27 + 0x60,
                 in_stack_fffffffffffffd48);
    }
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)
              (*(undefined8 *)(param_1 + 0x3f8),lVar25,lVar27 + 0x60);
    FUN_14011ae20(&lStack_f8,lVar27 + 0x40);
    FUN_1402f1fb0(&pcStack_1d8);
    lVar27 = lVar27 + 0xd8;
  } while( true );
}

