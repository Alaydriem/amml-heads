
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14541e320(longlong param_1,longlong *param_2,longlong *param_3,longlong *param_4)

{
  ulonglong uVar1;
  longlong lVar2;
  code *pcVar3;
  int iVar4;
  undefined4 uVar5;
  int iVar6;
  int iVar7;
  undefined4 uVar8;
  ulonglong uVar9;
  longlong *plVar10;
  longlong lVar11;
  undefined8 uVar12;
  longlong *plVar13;
  longlong lVar14;
  undefined8 *puVar15;
  int *piVar16;
  int *piVar17;
  int *piVar18;
  int *piVar19;
  int *piVar20;
  ulonglong uVar21;
  longlong *plVar22;
  longlong lVar23;
  int *piVar24;
  longlong *plVar25;
  ulonglong uVar26;
  ulonglong uVar27;
  bool bVar28;
  ulonglong in_stack_fffffffffffffdd8;
  undefined4 uStack_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined *puStack_1f8;
  undefined8 uStack_1f0;
  longlong *plStack_1e8;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  uint uStack_1cc;
  undefined8 uStack_1c8;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined4 uStack_1b8;
  undefined4 uStack_1b4;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined4 uStack_1a8;
  uint uStack_1a4;
  longlong lStack_1a0;
  undefined1 uStack_198;
  undefined1 auStack_190 [8];
  longlong lStack_188;
  int *piStack_160;
  int *piStack_158;
  int *piStack_150;
  longlong *plStack_148;
  longlong *plStack_140;
  longlong *plStack_138;
  undefined8 uStack_130;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  uint uStack_11c;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  uint uStack_f4;
  uint uStack_ec;
  undefined1 uStack_e8;
  longlong lStack_d8;
  longlong lStack_d0;
  int *piStack_c8;
  longlong *plStack_c0;
  longlong *plStack_b8;
  longlong *plStack_b0;
  longlong lStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  longlong *plStack_88;
  undefined8 *puStack_80;
  char cStack_78;
  longlong *plStack_70;
  longlong lStack_68;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  plStack_140 = param_3;
  plStack_c0 = param_2;
  plStack_70 = param_4;
  lStack_68 = param_1;
  FUN_14543e0d0(*param_4,param_4[1],(param_4[1] - *param_4 >> 3) * 0x6db6db6db6db6db7);
  piVar24 = (int *)plStack_70[1];
  piVar17 = (int *)*plStack_70;
  if ((int *)*plStack_70 != piVar24) {
    do {
      do {
        piVar18 = piVar17;
        piVar17 = piVar18 + 0xe;
        if (piVar17 == piVar24) goto LAB_14541e669;
      } while ((*(longlong *)(piVar18 + 10) != *(longlong *)(piVar18 + 0x18)) ||
              (*piVar18 != *piVar17));
      if (*(ulonglong *)(piVar18 + 8) < 0x10) {
        piVar16 = piVar18 + 2;
        if (0xf < *(ulonglong *)(piVar18 + 0x16)) goto LAB_14541e3fd;
LAB_14541e3e9:
        piVar19 = piVar18 + 0x10;
      }
      else {
        piVar16 = *(int **)(piVar18 + 2);
        if (*(ulonglong *)(piVar18 + 0x16) < 0x10) goto LAB_14541e3e9;
LAB_14541e3fd:
        piVar19 = *(int **)(piVar18 + 0x10);
      }
    } while ((*(longlong *)(piVar18 + 6) != *(longlong *)(piVar18 + 0x14)) ||
            ((*(longlong *)(piVar18 + 6) != 0 &&
             (iVar6 = FUN_14eec62b0(piVar16,piVar19), iVar6 != 0))));
    piVar16 = piVar18 + 0x1c;
    while (piVar19 = piVar16, piVar19 != piVar24) {
      iVar6 = *piVar19;
      if ((*(longlong *)(piVar18 + 10) == *(longlong *)(piVar17 + 0x18)) && (*piVar18 == iVar6)) {
        if (*(ulonglong *)(piVar18 + 8) < 0x10) {
          piVar16 = piVar18 + 2;
          if (0xf < *(ulonglong *)(piVar17 + 0x16)) goto LAB_14541e4a1;
LAB_14541e490:
          piVar20 = piVar17 + 0x10;
        }
        else {
          piVar16 = *(int **)(piVar18 + 2);
          if (*(ulonglong *)(piVar17 + 0x16) < 0x10) goto LAB_14541e490;
LAB_14541e4a1:
          piVar20 = *(int **)(piVar17 + 0x10);
        }
        if ((*(longlong *)(piVar18 + 6) != *(longlong *)(piVar17 + 0x14)) ||
           ((*(longlong *)(piVar18 + 6) != 0 && (iVar7 = FUN_14eec62b0(piVar16,piVar20), iVar7 != 0)
            ))) goto LAB_14541e4c0;
      }
      else {
LAB_14541e4c0:
        piVar18[0xe] = iVar6;
        if (piVar18 != piVar17) {
          if (*(ulonglong *)(piVar17 + 0x16) < 0x10) {
            piVar16 = piVar17 + 0x10;
          }
          else {
            piVar16 = *(int **)(piVar17 + 0x10);
          }
          piVar20 = piVar18 + 0x10;
          uVar21 = *(ulonglong *)(piVar17 + 0x14);
          if (*(ulonglong *)(piVar18 + 0x16) < uVar21) {
            FUN_14009d6d0(piVar20,uVar21);
          }
          else {
            if (0xf < *(ulonglong *)(piVar18 + 0x16)) {
              piVar20 = *(int **)(piVar18 + 0x10);
            }
            *(ulonglong *)(piVar18 + 0x14) = uVar21;
            FUN_14eec61e0(piVar20,piVar16,uVar21);
            *(undefined1 *)((longlong)piVar20 + uVar21) = 0;
          }
        }
        iVar6 = piVar17[0x19];
        iVar7 = piVar17[0x1a];
        iVar4 = piVar17[0x1b];
        piVar18[0x18] = piVar17[0x18];
        piVar18[0x19] = iVar6;
        piVar18[0x1a] = iVar7;
        piVar18[0x1b] = iVar4;
        piVar18 = piVar18 + 0xe;
      }
      piVar17 = piVar19;
      piVar16 = piVar19 + 0xe;
    }
    piVar18 = piVar18 + 0xe;
    if (piVar18 != piVar24) {
      piVar17 = (int *)plStack_70[1];
      piVar16 = piVar18;
      if (piVar17 != piVar24) {
        do {
          *piVar18 = *piVar24;
          if (piVar18 != piVar24) {
            if (*(ulonglong *)(piVar24 + 8) < 0x10) {
              piVar16 = piVar24 + 2;
            }
            else {
              piVar16 = *(int **)(piVar24 + 2);
            }
            piVar19 = piVar18 + 2;
            uVar21 = *(ulonglong *)(piVar24 + 6);
            if (*(ulonglong *)(piVar18 + 8) < uVar21) {
              FUN_14009d6d0(piVar19,uVar21);
            }
            else {
              if (0xf < *(ulonglong *)(piVar18 + 8)) {
                piVar19 = *(int **)(piVar18 + 2);
              }
              *(ulonglong *)(piVar18 + 6) = uVar21;
              FUN_14eec61e0(piVar19,piVar16,uVar21);
              *(undefined1 *)((longlong)piVar19 + uVar21) = 0;
            }
          }
          iVar6 = piVar24[0xb];
          iVar7 = piVar24[0xc];
          iVar4 = piVar24[0xd];
          piVar18[10] = piVar24[10];
          piVar18[0xb] = iVar6;
          piVar18[0xc] = iVar7;
          piVar18[0xd] = iVar4;
          piVar18 = piVar18 + 0xe;
          piVar24 = piVar24 + 0xe;
        } while (piVar24 != piVar17);
        piVar24 = (int *)plStack_70[1];
        piVar16 = piVar18;
      }
      for (; piVar18 != piVar24; piVar18 = piVar18 + 0xe) {
        uVar21 = *(ulonglong *)(piVar18 + 8);
        if (0xf < uVar21) {
          lVar11 = *(longlong *)(piVar18 + 2);
          uVar9 = uVar21 + 1;
          lVar23 = lVar11;
          if (0xfff < uVar9) {
            lVar23 = *(longlong *)(lVar11 + -8);
            if (0x1f < (ulonglong)((lVar11 + -8) - lVar23)) goto LAB_14541f208;
            uVar9 = uVar21 + 0x28;
          }
          FUN_14e934b00(lVar23,uVar9);
        }
        piVar18[6] = 0;
        piVar18[7] = 0;
        piVar18[8] = 0xf;
        piVar18[9] = 0;
        *(undefined1 *)(piVar18 + 2) = 0;
      }
      plStack_70[1] = (longlong)piVar16;
    }
  }
LAB_14541e669:
  plVar13 = *(longlong **)(lStack_68 + 0x1a8);
  plVar10 = (longlong *)*plVar13;
  if (plVar10 != plVar13) {
    plVar13 = (longlong *)(lStack_68 + 0x230);
    do {
      if ((int)plVar10[4] == 2) {
        FUN_145676fe0(plVar10 + 0xb);
        lVar11 = FUN_140ab6b70(plVar13,plVar10 + 4);
        if (lVar11 != *plVar13) {
          uVar12 = FUN_140132910(plVar13,lVar11);
          FUN_1411670c0(plVar13,uVar12);
        }
      }
      plVar25 = (longlong *)plVar10[2];
      plVar22 = plVar10;
      if (*(char *)(plVar10[2] + 0x19) == '\0') {
        do {
          plVar10 = plVar25;
          plVar25 = (longlong *)*plVar10;
        } while (*(char *)(*plVar10 + 0x19) == '\0');
      }
      else {
        do {
          plVar10 = (longlong *)plVar22[1];
          if (*(char *)((longlong)plVar10 + 0x19) != '\0') break;
          bVar28 = plVar22 == (longlong *)plVar10[2];
          plVar22 = plVar10;
        } while (bVar28);
      }
    } while (plVar10 != *(longlong **)(lStack_68 + 0x1a8));
    plVar13 = plVar10;
    plVar10 = (longlong *)**(longlong **)(lStack_68 + 0x1a8);
  }
  piVar24 = (int *)plStack_c0[1];
  piVar17 = piVar24 + *plStack_c0 * 0x1c;
  plStack_148 = (longlong *)(lStack_68 + 0x230);
  piStack_158 = piVar24;
  piStack_150 = piVar17;
  do {
    if (plVar10 == plVar13) {
LAB_14541ead0:
      lVar11 = plStack_140[1];
      lVar23 = *plStack_140 * 0x38 + lVar11;
      plStack_b8 = (longlong *)(lStack_68 + 0x1d8);
      do {
        if (lVar11 == lVar23) {
          uStack_208 = (undefined4)*plStack_c0;
          uStack_204 = *(undefined4 *)((longlong)plStack_c0 + 4);
          uStack_200 = (undefined4)plStack_c0[1];
          uStack_1fc = *(undefined4 *)((longlong)plStack_c0 + 0xc);
          FUN_14541e1e0(lStack_68,&uStack_208);
          FUN_1408e8c10(plStack_70);
          return;
        }
        uStack_1d8 = 0;
        uStack_1d4 = 0;
        uStack_1d0 = 0;
        uStack_1cc = uStack_1cc & 0xff000000;
        plStack_1e8 = (longlong *)0x0;
        uStack_1e0 = 0;
        uStack_1dc = 0;
        uStack_1c0 = 1;
        uStack_1ac = 0;
        uStack_1a8 = 0;
        uStack_1a4 = uStack_1a4 & 0xffffff00;
        uStack_1bc = 0;
        uStack_1b8 = 0;
        uStack_1b4 = 0;
        uStack_1b0 = 0;
        lStack_1a0 = CONCAT35(lStack_1a0._5_3_,8);
        uStack_198 = 1;
        uStack_1c8 = 1;
        if (*(longlong *)(lStack_68 + 0x210) == 0) {
          lVar14 = 0;
        }
        else {
          LOCK();
          piVar24 = (int *)(*(longlong *)(lStack_68 + 0x210) + 8);
          *piVar24 = *piVar24 + 1;
          UNLOCK();
          lVar14 = *(longlong *)(lStack_68 + 0x210);
        }
        lVar2 = *(longlong *)(lStack_68 + 0x208);
        lStack_d8 = lVar2;
        lStack_d0 = lVar14;
        if (lVar2 == 0) {
          FUN_1454208b0(&plStack_138,lVar11,0);
          if ((char)uStack_108 == '\x01') {
            FUN_145a4bd30(auStack_190,&plStack_138);
            lStack_a8 = 0;
            uStack_a0 = 0;
            uStack_98 = 0;
            lVar14 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,0x30);
            if (lVar14 == 0) {
              FUN_1409bc820(0x30);
            }
            plStack_88 = &lStack_a8;
            lStack_a8 = lVar14;
            uStack_a0 = lVar14;
            uStack_98 = lVar14 + 0x30;
            FUN_145a4c3c0(lVar14,auStack_190);
            uStack_a0 = lVar14 + 0x30;
            puVar15 = (undefined8 *)
                      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,0x30);
            if (puVar15 == (undefined8 *)0x0) {
              FUN_1409bc820(0x30);
            }
            lVar14 = uStack_a0;
            puVar15[1] = 0x100000001;
            *puVar15 = &UNK_14effcbe0;
            plStack_88 = puVar15 + 2;
            puVar15[2] = &UNK_14f273420;
            puVar15[3] = 0;
            puVar15[4] = 0;
            puVar15[5] = 0;
            puStack_80 = puVar15;
            if (puVar15 + 3 == &lStack_a8) {
              cStack_78 = '\x01';
              lVar2 = lStack_a8;
              if (lStack_a8 != 0) {
                for (; lVar2 != lVar14; lVar2 = lVar2 + 0x30) {
                  if (*(longlong *)(lVar2 + 8) != 0) {
                    (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
                  }
                }
                uVar21 = uStack_98 - lStack_a8;
                lVar14 = lStack_a8;
                if (0xfff < uVar21) {
                  lVar14 = *(longlong *)(lStack_a8 + -8);
                  if (0x1f < (ulonglong)((lStack_a8 + -8) - lVar14)) {
LAB_14541f208:
                    _invoke_watson(0,0,0,0,0);
                    pcVar3 = (code *)swi(3);
                    (*pcVar3)();
                    return;
                  }
                  uVar21 = uVar21 + 0x27;
                }
                FUN_14e934b00(lVar14,uVar21);
                lStack_a8 = 0;
                uStack_a0 = 0;
                uStack_98 = 0;
              }
            }
            else {
              puVar15[3] = lStack_a8;
              *(undefined4 *)(puVar15 + 4) = (undefined4)uStack_a0;
              *(undefined4 *)((longlong)puVar15 + 0x24) = uStack_a0._4_4_;
              *(undefined4 *)(puVar15 + 5) = (undefined4)uStack_98;
              *(undefined4 *)((longlong)puVar15 + 0x2c) = uStack_98._4_4_;
              lStack_a8 = 0;
              uStack_a0 = 0;
              uStack_98 = 0;
              cStack_78 = '\x01';
            }
            if (lStack_188 != 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
            }
            lVar14 = lStack_d0;
            if (((char)uStack_108 == '\x01') && (uStack_130 != (undefined8 *)0x0)) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
              lVar14 = lStack_d0;
            }
          }
          else {
            cStack_78 = '\0';
            lVar14 = lStack_d0;
            plStack_88 = plStack_138;
            puStack_80 = uStack_130;
          }
        }
        else {
          uVar8 = FUN_145a2d440(lVar11);
          in_stack_fffffffffffffdd8 = in_stack_fffffffffffffdd8 & 0xffffffffffffff00;
          FUN_141676a20(lVar2,&plStack_138,lVar11,uVar8,in_stack_fffffffffffffdd8,0,0);
          cStack_78 = (char)uStack_128 != '\0';
          plStack_88 = plStack_138;
          puStack_80 = uStack_130;
        }
        if (lVar14 != 0) {
          LOCK();
          piVar24 = (int *)(lVar14 + 8);
          *piVar24 = *piVar24 + -1;
          UNLOCK();
          if (*piVar24 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar14);
            LOCK();
            piVar24 = (int *)(lVar14 + 0xc);
            *piVar24 = *piVar24 + -1;
            UNLOCK();
            if (*piVar24 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar14);
            }
          }
        }
        if (cStack_78 == '\x01') {
          FUN_144df9340(&plStack_138,&plStack_88,1);
          plStack_1e8 = plStack_138;
          uVar8 = (undefined4)uStack_130;
          uVar5 = uStack_130._4_4_;
          plStack_138 = (longlong *)0x0;
          uStack_130 = (undefined8 *)0x0;
          lVar14 = CONCAT44(uStack_1dc,uStack_1e0);
          uStack_1e0 = uVar8;
          uStack_1dc = uVar5;
          if (lVar14 == 0) {
            lStack_1a0 = (ulonglong)uStack_ec << 0x20;
            uStack_1a8 = uStack_f8;
            uStack_1a4 = uStack_f4;
            uStack_1b8 = uStack_108;
            uStack_1b4 = uStack_104;
            uStack_1b0 = uStack_100;
            uStack_1ac = uStack_fc;
            uStack_1c0 = uStack_110;
            uStack_1bc = uStack_10c;
            uStack_1d8 = uStack_128;
            uStack_1d4 = uStack_124;
            uStack_1d0 = uStack_120;
            uStack_1cc = uStack_11c;
            uStack_198 = uStack_e8;
          }
          else {
            LOCK();
            piVar24 = (int *)(lVar14 + 8);
            *piVar24 = *piVar24 + -1;
            UNLOCK();
            if (*piVar24 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar14);
              LOCK();
              piVar24 = (int *)(lVar14 + 0xc);
              *piVar24 = *piVar24 + -1;
              UNLOCK();
              if (*piVar24 == 0) {
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar14);
              }
            }
            puVar15 = uStack_130;
            uStack_1a8 = uStack_f8;
            uStack_1a4 = uStack_f4;
            lStack_1a0 = (ulonglong)uStack_ec << 0x20;
            uStack_1b8 = uStack_108;
            uStack_1b4 = uStack_104;
            uStack_1b0 = uStack_100;
            uStack_1ac = uStack_fc;
            uStack_1c0 = uStack_110;
            uStack_1bc = uStack_10c;
            uStack_1d8 = uStack_128;
            uStack_1d4 = uStack_124;
            uStack_1d0 = uStack_120;
            uStack_1cc = uStack_11c;
            uStack_198 = uStack_e8;
            if (uStack_130 != (undefined8 *)0x0) {
              LOCK();
              piVar24 = (int *)(uStack_130 + 1);
              *piVar24 = *piVar24 + -1;
              UNLOCK();
              if (*piVar24 == 0) {
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_130);
                LOCK();
                piVar24 = (int *)((longlong)puVar15 + 0xc);
                *piVar24 = *piVar24 + -1;
                UNLOCK();
                if (*piVar24 == 0) {
                  (*(code *)PTR__guard_dispatch_icall_14f99ef70)(puVar15);
                }
              }
            }
          }
        }
        else {
          FUN_145a4c3c0(auStack_190,plStack_b8);
          FUN_144df8eb0(&plStack_138,auStack_190,uStack_198);
          plStack_1e8 = plStack_138;
          uVar8 = (undefined4)uStack_130;
          uVar5 = uStack_130._4_4_;
          plStack_138 = (longlong *)0x0;
          uStack_130 = (undefined8 *)0x0;
          lVar14 = CONCAT44(uStack_1dc,uStack_1e0);
          uStack_1e0 = uVar8;
          uStack_1dc = uVar5;
          if (lVar14 == 0) {
            lStack_1a0 = (ulonglong)uStack_ec << 0x20;
            uStack_1a8 = uStack_f8;
            uStack_1a4 = uStack_f4;
            uStack_1b8 = uStack_108;
            uStack_1b4 = uStack_104;
            uStack_1b0 = uStack_100;
            uStack_1ac = uStack_fc;
            uStack_1c0 = uStack_110;
            uStack_1bc = uStack_10c;
            uStack_1d8 = uStack_128;
            uStack_1d4 = uStack_124;
            uStack_1d0 = uStack_120;
            uStack_1cc = uStack_11c;
            uStack_198 = uStack_e8;
          }
          else {
            LOCK();
            piVar24 = (int *)(lVar14 + 8);
            *piVar24 = *piVar24 + -1;
            UNLOCK();
            if (*piVar24 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar14);
              LOCK();
              piVar24 = (int *)(lVar14 + 0xc);
              *piVar24 = *piVar24 + -1;
              UNLOCK();
              if (*piVar24 == 0) {
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar14);
              }
            }
            puVar15 = uStack_130;
            uStack_1a8 = uStack_f8;
            uStack_1a4 = uStack_f4;
            lStack_1a0 = (ulonglong)uStack_ec << 0x20;
            uStack_1b8 = uStack_108;
            uStack_1b4 = uStack_104;
            uStack_1b0 = uStack_100;
            uStack_1ac = uStack_fc;
            uStack_1c0 = uStack_110;
            uStack_1bc = uStack_10c;
            uStack_1d8 = uStack_128;
            uStack_1d4 = uStack_124;
            uStack_1d0 = uStack_120;
            uStack_1cc = uStack_11c;
            uStack_198 = uStack_e8;
            if (uStack_130 != (undefined8 *)0x0) {
              LOCK();
              piVar24 = (int *)(uStack_130 + 1);
              *piVar24 = *piVar24 + -1;
              UNLOCK();
              if (*piVar24 == 0) {
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_130);
                LOCK();
                piVar24 = (int *)((longlong)puVar15 + 0xc);
                *piVar24 = *piVar24 + -1;
                UNLOCK();
                if (*piVar24 == 0) {
                  (*(code *)PTR__guard_dispatch_icall_14f99ef70)(puVar15);
                }
              }
            }
          }
          if (lStack_188 != 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
          }
        }
        puVar15 = puStack_80;
        if ((cStack_78 == '\x01') && (puStack_80 != (undefined8 *)0x0)) {
          LOCK();
          piVar24 = (int *)(puStack_80 + 1);
          *piVar24 = *piVar24 + -1;
          UNLOCK();
          if (*piVar24 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(puStack_80);
            LOCK();
            piVar24 = (int *)((longlong)puVar15 + 0xc);
            *piVar24 = *piVar24 + -1;
            UNLOCK();
            if (*piVar24 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(puVar15);
            }
          }
        }
        puStack_1f8 = &UNK_14f8469d4;
        uStack_1f0 = 0;
        FUN_145420ec0(lStack_68,lVar11);
        lVar14 = CONCAT44(uStack_1dc,uStack_1e0);
        if (lVar14 != 0) {
          LOCK();
          piVar24 = (int *)(lVar14 + 8);
          *piVar24 = *piVar24 + -1;
          UNLOCK();
          if (*piVar24 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar14);
            LOCK();
            piVar24 = (int *)(lVar14 + 0xc);
            *piVar24 = *piVar24 + -1;
            UNLOCK();
            if (*piVar24 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar14);
            }
          }
        }
        lVar11 = lVar11 + 0x38;
      } while( true );
    }
    piVar18 = (int *)plStack_c0[1] + *plStack_c0 * 0x1c;
    if ((piVar17 != piVar18 || (int *)plStack_c0[1] != piStack_158) || (piVar18 < piVar24)) {
      FUN_140078ed0();
      goto LAB_14541ead0;
    }
    piStack_c8 = piVar24;
    plStack_b0 = plVar10;
    if (0 < (longlong)piVar17 - (longlong)piVar24) {
      if ((ulonglong)plVar10[8] < 0x10) {
        plVar13 = plVar10 + 5;
      }
      else {
        plVar13 = (longlong *)plVar10[5];
      }
      uVar21 = plVar10[7];
      uVar9 = ((ulonglong)((longlong)piVar17 - (longlong)piVar24) >> 4) * 0x6db6db6db6db6db7;
      do {
        uVar27 = uVar9 >> 1;
        if (*(ulonglong *)(piVar24 + uVar27 * 0x1c + 8) < 0x10) {
          piVar17 = piVar24 + uVar27 * 0x1c + 2;
        }
        else {
          piVar17 = *(int **)(piVar24 + uVar27 * 0x1c + 2);
        }
        uVar1 = *(ulonglong *)(piVar24 + uVar27 * 0x1c + 6);
        uVar26 = uVar21;
        if (uVar1 <= uVar21) {
          uVar26 = uVar1;
        }
        iVar6 = FUN_14eec62b0(piVar17,plVar13,uVar26);
        bVar28 = uVar1 < uVar21;
        if (iVar6 != 0) {
          bVar28 = iVar6 < 0;
        }
        if (bVar28) {
          piVar24 = piVar24 + uVar27 * 0x1c + 0x1c;
          uVar27 = ~uVar27 + uVar9;
        }
        uVar9 = uVar27;
      } while (0 < (longlong)uVar27);
    }
    piVar18 = (int *)*plStack_70;
    piStack_160 = (int *)plStack_70[1];
    if (0 < (longlong)piStack_160 - (longlong)piVar18) {
      if ((ulonglong)plStack_b0[8] < 0x10) {
        plStack_b8 = plStack_b0 + 5;
      }
      else {
        plStack_b8 = (longlong *)plStack_b0[5];
      }
      uVar21 = plStack_b0[7];
      uVar9 = ((ulonglong)((longlong)piStack_160 - (longlong)piVar18) >> 3) * 0x6db6db6db6db6db7;
      do {
        uVar27 = uVar9 >> 1;
        if (*(ulonglong *)(piVar18 + uVar27 * 0xe + 8) < 0x10) {
          piVar17 = piVar18 + uVar27 * 0xe + 2;
        }
        else {
          piVar17 = *(int **)(piVar18 + uVar27 * 0xe + 2);
        }
        uVar1 = *(ulonglong *)(piVar18 + uVar27 * 0xe + 6);
        uVar26 = uVar21;
        if (uVar1 <= uVar21) {
          uVar26 = uVar1;
        }
        iVar6 = FUN_14eec62b0(piVar17,plStack_b8,uVar26);
        bVar28 = uVar1 < uVar21;
        if (iVar6 != 0) {
          bVar28 = iVar6 < 0;
        }
        if (bVar28) {
          piVar18 = piVar18 + uVar27 * 0xe + 0xe;
          uVar27 = ~uVar27 + uVar9;
        }
        uVar9 = uVar27;
      } while (0 < (longlong)uVar27);
    }
    plVar25 = plStack_b0;
    piVar17 = piStack_150;
    plVar13 = plStack_b0 + 4;
    if (((piVar18 == piStack_160) || (*(longlong *)(piVar18 + 10) != plStack_b0[9])) ||
       (*piVar18 != (int)*plVar13)) {
LAB_14541e9d0:
      if ((piVar24 != piVar17) &&
         ((*(longlong *)(piVar24 + 10) == plVar25[9] && (*piVar24 == (int)*plVar13)))) {
        if (*(ulonglong *)(piVar24 + 8) < 0x10) {
          piVar18 = piVar24 + 2;
          if (0xf < (ulonglong)plVar25[8]) goto LAB_14541ea0f;
LAB_14541e9fd:
          plVar10 = plVar25 + 5;
        }
        else {
          piVar18 = *(int **)(piVar24 + 2);
          if ((ulonglong)plVar25[8] < 0x10) goto LAB_14541e9fd;
LAB_14541ea0f:
          plVar10 = (longlong *)plVar25[5];
        }
        if ((*(longlong *)(piVar24 + 6) == plVar25[7]) &&
           ((*(longlong *)(piVar24 + 6) == 0 || (iVar6 = FUN_14eec62b0(piVar18,plVar10), iVar6 == 0)
            ))) goto LAB_14541ea76;
      }
      FUN_145676fe0(plVar25 + 0xb);
      plVar10 = plStack_148;
      lVar11 = FUN_140ab6b70(plStack_148,plVar13);
      piVar24 = piStack_c8;
      plVar25 = plStack_b0;
      if (lVar11 != *plVar10) {
        uVar12 = FUN_140132910(plVar10,lVar11);
        FUN_1411670c0(plVar10,uVar12);
        plVar25 = plStack_b0;
      }
    }
    else {
      if (*(ulonglong *)(piVar18 + 8) < 0x10) {
        piVar16 = piVar18 + 2;
        if (0xf < (ulonglong)plStack_b0[8]) goto LAB_14541e9aa;
LAB_14541e999:
        plVar10 = plStack_b0 + 5;
      }
      else {
        piVar16 = *(int **)(piVar18 + 2);
        if ((ulonglong)plStack_b0[8] < 0x10) goto LAB_14541e999;
LAB_14541e9aa:
        plVar10 = (longlong *)plStack_b0[5];
      }
      if ((*(longlong *)(piVar18 + 6) != plStack_b0[7]) ||
         ((*(longlong *)(piVar18 + 6) != 0 && (iVar6 = FUN_14eec62b0(piVar16,plVar10), iVar6 != 0)))
         ) goto LAB_14541e9d0;
    }
LAB_14541ea76:
    plVar13 = (longlong *)plVar25[2];
    if (*(char *)(plVar25[2] + 0x19) == '\0') {
      do {
        plVar10 = plVar13;
        plVar13 = (longlong *)*plVar10;
      } while (*(char *)(*plVar10 + 0x19) == '\0');
    }
    else {
      do {
        plVar10 = (longlong *)plVar25[1];
        if (*(char *)((longlong)plVar10 + 0x19) != '\0') break;
        bVar28 = plVar25 == (longlong *)plVar10[2];
        plVar25 = plVar10;
      } while (bVar28);
    }
    plVar13 = *(longlong **)(lStack_68 + 0x1a8);
  } while( true );
}

