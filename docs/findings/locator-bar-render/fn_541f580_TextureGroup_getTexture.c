
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong *
FUN_14541f580(ulonglong *param_1,ulonglong *param_2,undefined4 *param_3,char param_4,
             ulonglong param_5,byte param_6)

{
  int *piVar1;
  char cVar2;
  longlong *plVar3;
  code *pcVar4;
  bool bVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  uint uVar9;
  int iVar10;
  ulonglong *puVar11;
  ulonglong uVar12;
  longlong lVar13;
  ulonglong *puVar14;
  ulonglong *puVar15;
  undefined4 *puVar16;
  ulonglong *puVar17;
  ulonglong uVar18;
  longlong *plVar19;
  ulonglong uVar20;
  ulonglong *unaff_RBX;
  ulonglong *puVar21;
  undefined8 *puVar22;
  longlong *plVar23;
  ulonglong *puVar24;
  bool bVar25;
  ulonglong *unaff_R13;
  undefined4 *puVar26;
  longlong unaff_GS_OFFSET;
  undefined1 auStack_168 [8];
  longlong lStack_160;
  ulonglong *puStack_138;
  longlong lStack_130;
  undefined *puStack_e0;
  undefined8 uStack_d8;
  ulonglong *puStack_d0;
  ulonglong *puStack_c8;
  undefined4 auStack_c0 [2];
  ulonglong uStack_b8;
  ulonglong uStack_b0;
  ulonglong uStack_a8;
  ulonglong uStack_a0;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  ulonglong *puStack_80;
  longlong lStack_78;
  ulonglong *puStack_70;
  ulonglong *puStack_68;
  undefined4 *puStack_60;
  ulonglong *puStack_58;
  char cStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  puStack_80 = param_1;
  cStack_49 = param_4;
  FUN_1454756c0(param_1 + 0x32,&puStack_138);
  puVar15 = puStack_138;
  puVar11 = (ulonglong *)FUN_140ab6b70(param_1 + 0x41,param_3);
  puVar21 = (ulonglong *)param_1[0x41];
  puStack_70 = param_2;
  puStack_60 = param_3;
  if (puVar11 == puVar21) {
    puStack_68 = puVar15;
    puVar15 = (ulonglong *)puVar21[1];
    bVar25 = true;
    unaff_RBX = puVar21;
    if (*(char *)((longlong)puVar15 + 0x19) == '\0') {
      if (*(ulonglong *)(param_3 + 8) < 0x10) {
        puVar16 = param_3 + 2;
      }
      else {
        puVar16 = *(undefined4 **)(param_3 + 2);
      }
      unaff_R13 = *(ulonglong **)(param_3 + 6);
      puVar11 = puVar15;
      do {
        puVar15 = puVar11;
        if (puVar15[8] < 0x10) {
          puVar11 = puVar15 + 5;
        }
        else {
          puVar11 = (ulonglong *)puVar15[5];
        }
        puVar17 = (ulonglong *)puVar15[7];
        puVar14 = puVar17;
        if (unaff_R13 < puVar17) {
          puVar14 = unaff_R13;
        }
        iVar10 = FUN_14eec62b0(puVar11,puVar16,puVar14);
        bVar25 = puVar17 < unaff_R13;
        if (iVar10 != 0) {
          bVar25 = iVar10 < 0;
        }
        if (bVar25 == false) {
          unaff_RBX = puVar15;
        }
        puVar11 = (ulonglong *)puVar15[(ulonglong)bVar25 * 2];
      } while (*(char *)((longlong)puVar15[(ulonglong)bVar25 * 2] + 0x19) == '\0');
    }
    if (*(char *)((longlong)unaff_RBX + 0x19) == '\0') {
      if (*(ulonglong *)(puStack_60 + 8) < 0x10) {
        puVar16 = puStack_60 + 2;
      }
      else {
        puVar16 = *(undefined4 **)(puStack_60 + 2);
      }
      uVar12 = *(ulonglong *)(puStack_60 + 6);
      uVar18 = unaff_RBX[7];
      if (unaff_RBX[8] < 0x10) {
        puVar11 = unaff_RBX + 5;
      }
      else {
        puVar11 = (ulonglong *)unaff_RBX[5];
      }
      uVar20 = uVar12;
      if (uVar18 < uVar12) {
        uVar20 = uVar18;
      }
      iVar10 = FUN_14eec62b0(puVar16,puVar11,uVar20);
      bVar5 = uVar18 <= uVar12;
      if (iVar10 != 0) {
        bVar5 = -1 < iVar10;
      }
      unaff_R13 = unaff_RBX;
      if (!bVar5) goto LAB_14541f732;
LAB_14541f915:
      unaff_R13[0xb] = param_5;
      puVar15 = puStack_68;
      goto LAB_14541f927;
    }
LAB_14541f732:
    puVar16 = puStack_60;
    unaff_RBX = puStack_80;
    if (puStack_80[0x42] == 0x2aaaaaaaaaaaaaa) {
      FUN_1400a4ea0();
    }
    else {
      unaff_R13 = (ulonglong *)(*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,0x60);
      if (unaff_R13 == (ulonglong *)0x0) {
        FUN_1409bc820(0x60);
      }
      *(undefined4 *)(unaff_R13 + 4) = *puVar16;
      unaff_R13[5] = 0;
      unaff_R13[6] = 0;
      unaff_R13[7] = 0;
      unaff_R13[8] = 0;
      uVar12 = *(ulonglong *)(puVar16 + 6);
      puVar11 = unaff_RBX + 0x41;
      if (*(ulonglong *)(puVar16 + 8) < 0x10) {
        puVar17 = (ulonglong *)(puVar16 + 2);
      }
      else {
        puVar17 = *(ulonglong **)(puVar16 + 2);
      }
      if (-1 < (longlong)uVar12) {
        if (uVar12 < 0x10) {
          unaff_R13[7] = uVar12;
          unaff_R13[8] = 0xf;
          uVar12 = puVar17[1];
          unaff_R13[5] = *puVar17;
          unaff_R13[6] = uVar12;
        }
        else {
          uVar20 = uVar12 | 0xf;
          uVar18 = 0x16;
          if (0x16 < uVar20) {
            uVar18 = uVar20;
          }
          puStack_d0 = puVar17;
          puStack_c8 = puVar11;
          puStack_58 = unaff_R13;
          if (uVar20 < 0xfff) {
            lStack_78 = uVar18 + 1;
            uVar20 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
            if (uVar20 == 0) {
              FUN_1409bc820(lStack_78);
              uVar20 = 0;
            }
          }
          else {
            lStack_78 = uVar18 + 0x28;
            lVar13 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
            if (lVar13 == 0) {
              FUN_1409bc820(lStack_78);
            }
            uVar20 = lVar13 + 0x27U & 0xffffffffffffffe0;
            *(longlong *)(uVar20 - 8) = lVar13;
          }
          unaff_R13 = puStack_58;
          puStack_58[5] = uVar20;
          puStack_58[7] = uVar12;
          puStack_58[8] = uVar18;
          FUN_14eec62c0(uVar20,puStack_d0,uVar12 + 1);
          puVar11 = puStack_c8;
        }
        uVar6 = puStack_60[0xb];
        uVar7 = puStack_60[0xc];
        uVar8 = puStack_60[0xd];
        *(undefined4 *)(unaff_R13 + 9) = puStack_60[10];
        *(undefined4 *)((longlong)unaff_R13 + 0x4c) = uVar6;
        *(undefined4 *)(unaff_R13 + 10) = uVar7;
        *(undefined4 *)((longlong)unaff_R13 + 0x54) = uVar8;
        *(undefined1 *)((longlong)unaff_R13 + 0x5c) = 0;
        *unaff_R13 = (ulonglong)puVar21;
        unaff_R13[1] = (ulonglong)puVar21;
        unaff_R13[2] = (ulonglong)puVar21;
        *(undefined2 *)(unaff_R13 + 3) = 0;
        unaff_RBX[0x42] = unaff_RBX[0x42] + 1;
        puVar21 = (ulonglong *)unaff_RBX[0x41];
        unaff_R13[1] = (ulonglong)puVar15;
        if (puVar15 == puVar21) {
          *puVar21 = (ulonglong)unaff_R13;
          puVar21[1] = (ulonglong)unaff_R13;
          puVar21[2] = (ulonglong)unaff_R13;
          puVar15 = unaff_R13;
        }
        else {
          puVar17 = unaff_R13 + 1;
          if (bVar25 == false) {
            *puVar15 = (ulonglong)unaff_R13;
            if (puVar15 == (ulonglong *)*puVar21) {
              *puVar21 = (ulonglong)unaff_R13;
            }
          }
          else {
            puVar15[2] = (ulonglong)unaff_R13;
            if (puVar15 == (ulonglong *)puVar21[2]) {
              puVar21[2] = (ulonglong)unaff_R13;
            }
          }
          puVar15 = (ulonglong *)*puVar17;
          cVar2 = (char)puVar15[3];
          puVar14 = unaff_R13;
          while (cVar2 == '\0') {
            puVar22 = (undefined8 *)puVar15[1];
            puVar24 = (ulonglong *)*puVar22;
            if (puVar15 == puVar24) {
              puVar24 = (ulonglong *)puVar22[2];
              if ((char)puVar24[3] != '\0') {
                puVar17 = (ulonglong *)puVar15[2];
                if (puVar14 == puVar17) {
                  uVar12 = *puVar17;
                  puVar15[2] = uVar12;
                  if (*(char *)(uVar12 + 0x19) == '\0') {
                    *(ulonglong **)(uVar12 + 8) = puVar15;
                    puVar22 = (undefined8 *)puVar15[1];
                  }
                  puVar17[1] = (ulonglong)puVar22;
                  if (puVar15 == *(ulonglong **)(*puVar11 + 8)) {
                    *(ulonglong **)(*puVar11 + 8) = puVar17;
                  }
                  else if (puVar15 == (ulonglong *)*puVar22) {
                    *puVar22 = puVar17;
                  }
                  else {
                    puVar22[2] = puVar17;
                  }
                  *puVar17 = (ulonglong)puVar15;
                  puVar15[1] = (ulonglong)puVar17;
                  puVar14 = puVar15;
                  puVar15 = puVar17;
                }
                *(undefined1 *)(puVar15 + 3) = 1;
                *(undefined1 *)(*(longlong *)(puVar14[1] + 8) + 0x18) = 0;
                plVar19 = *(longlong **)(puVar14[1] + 8);
                plVar23 = (longlong *)*plVar19;
                lVar13 = plVar23[2];
                *plVar19 = lVar13;
                if (*(char *)(lVar13 + 0x19) == '\0') {
                  *(longlong **)(lVar13 + 8) = plVar19;
                }
                puVar22 = (undefined8 *)plVar19[1];
                plVar23[1] = (longlong)puVar22;
                if (plVar19 == *(longlong **)(*puVar11 + 8)) {
                  *(longlong **)(*puVar11 + 8) = plVar23;
                  plVar23[2] = (longlong)plVar19;
                }
                else if (plVar19 == (longlong *)puVar22[2]) {
                  puVar22[2] = plVar23;
                  plVar23[2] = (longlong)plVar19;
                }
                else {
                  *puVar22 = plVar23;
                  plVar23[2] = (longlong)plVar19;
                }
                goto LAB_14541fbb3;
              }
LAB_14541fa70:
              *(undefined1 *)(puVar15 + 3) = 1;
              *(undefined1 *)(puVar24 + 3) = 1;
              *(undefined1 *)(*(longlong *)(*puVar17 + 8) + 0x18) = 0;
              puVar14 = *(ulonglong **)(*puVar17 + 8);
            }
            else {
              if ((char)puVar24[3] == '\0') goto LAB_14541fa70;
              puVar17 = (ulonglong *)*puVar15;
              if (puVar14 == puVar17) {
                uVar12 = puVar17[2];
                *puVar15 = uVar12;
                if (*(char *)(uVar12 + 0x19) == '\0') {
                  *(ulonglong **)(uVar12 + 8) = puVar15;
                  puVar22 = (undefined8 *)puVar15[1];
                }
                puVar17[1] = (ulonglong)puVar22;
                if (puVar15 == *(ulonglong **)(*puVar11 + 8)) {
                  *(ulonglong **)(*puVar11 + 8) = puVar17;
                }
                else if (puVar15 == (ulonglong *)puVar22[2]) {
                  puVar22[2] = puVar17;
                }
                else {
                  *puVar22 = puVar17;
                }
                puVar17[2] = (ulonglong)puVar15;
                puVar15[1] = (ulonglong)puVar17;
                puVar14 = puVar15;
                puVar15 = puVar17;
              }
              *(undefined1 *)(puVar15 + 3) = 1;
              *(undefined1 *)(*(longlong *)(puVar14[1] + 8) + 0x18) = 0;
              plVar19 = *(longlong **)(puVar14[1] + 8);
              plVar23 = (longlong *)plVar19[2];
              lVar13 = *plVar23;
              plVar19[2] = lVar13;
              if (*(char *)(lVar13 + 0x19) == '\0') {
                *(longlong **)(lVar13 + 8) = plVar19;
              }
              plVar3 = (longlong *)plVar19[1];
              plVar23[1] = (longlong)plVar3;
              if (plVar19 == *(longlong **)(*puVar11 + 8)) {
                *(longlong **)(*puVar11 + 8) = plVar23;
              }
              else if (plVar19 == (longlong *)*plVar3) {
                *plVar3 = (longlong)plVar23;
              }
              else {
                plVar3[2] = (longlong)plVar23;
              }
              *plVar23 = (longlong)plVar19;
LAB_14541fbb3:
              plVar19[1] = (longlong)plVar23;
            }
            puVar17 = puVar14 + 1;
            puVar15 = (ulonglong *)puVar14[1];
            cVar2 = (char)puVar15[3];
          }
          puVar15 = (ulonglong *)puVar21[1];
        }
        *(undefined1 *)(puVar15 + 3) = 1;
        goto LAB_14541f915;
      }
    }
    puStack_58 = unaff_R13;
    FUN_140075720();
LAB_1454201db:
    _invoke_watson(0,0,0,0,0);
LAB_1454201f4:
    FUN_1409bc820(unaff_RBX);
    pcVar4 = (code *)swi(3);
    puVar15 = (ulonglong *)(*pcVar4)();
    return puVar15;
  }
  if (((param_5 >> 0x20 & 1) != 0) &&
     ((*(char *)((longlong)puVar11 + 0x5c) != '\x01' || ((uint)puVar11[0xb] < (uint)param_5)))) {
    puVar11[0xb] = param_5;
  }
LAB_14541f927:
  puVar16 = puStack_60;
  puVar21 = puVar15 + 0xb;
  uVar9 = param_6 - 8;
  puStack_68 = (ulonglong *)CONCAT44(puStack_68._4_4_,uVar9);
  if (uVar9 < 2) {
    puVar11 = puVar15 + 0xf;
  }
  else {
    puVar11 = puVar21;
    if (param_6 - 2 < 2) {
      puVar11 = puVar15 + 0xd;
    }
  }
  puVar17 = puVar21;
  if (*(longlong *)*puVar11 == 0) goto LAB_14541f9cf;
  if (uVar9 < 2) {
    puVar11 = puVar15 + 0xf;
  }
  else {
    puVar11 = puVar21;
    if (param_6 - 2 < 2) {
      puVar11 = puVar15 + 0xd;
    }
  }
  unaff_RBX = *(ulonglong **)*puVar11;
  if (unaff_RBX == (ulonglong *)0x0) goto LAB_14541f9cf;
  if (*(int *)(*(longlong *)(*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)__tls_index * 8) + 4
              ) < _DAT_1528d6fc8) goto LAB_145420147;
  while( true ) {
    puVar21 = puVar15;
    LOCK();
    *(short *)(unaff_RBX + 2) = (short)unaff_RBX[2] + 1;
    UNLOCK();
    puVar15 = puVar21;
    if ((cStack_49 == '\0') && ((*(byte *)((longlong)unaff_RBX + 0x12) & 1) != 0))
    goto LAB_14541ff87;
LAB_14541f9cf:
    puVar21 = puVar15;
    puVar15 = puStack_80;
    uVar12 = FUN_140ab6b70(puStack_80 + 0x43);
    if (uVar12 == puVar15[0x43]) {
      puVar11 = (ulonglong *)0x0;
    }
    else if (*(longlong *)(uVar12 + 0x60) == 0) {
      puVar11 = *(ulonglong **)(uVar12 + 0x58);
    }
    else {
      LOCK();
      piVar1 = (int *)(*(longlong *)(uVar12 + 0x60) + 8);
      *piVar1 = *piVar1 + 1;
      UNLOCK();
      puVar11 = *(ulonglong **)(uVar12 + 0x58);
      unaff_RBX = *(ulonglong **)(uVar12 + 0x60);
      if (unaff_RBX != (ulonglong *)0x0) {
        LOCK();
        puVar15 = unaff_RBX + 1;
        *(int *)puVar15 = (int)*puVar15 + -1;
        UNLOCK();
        if ((int)*puVar15 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(unaff_RBX);
          LOCK();
          piVar1 = (int *)((longlong)unaff_RBX + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(unaff_RBX);
          }
        }
      }
    }
    auStack_c0[0] = *puVar16;
    uStack_b8 = 0;
    uStack_b0 = 0;
    uStack_a8 = 0;
    uStack_a0 = 0;
    uVar12 = *(ulonglong *)(puVar16 + 6);
    if (*(ulonglong *)(puVar16 + 8) < 0x10) {
      puVar15 = (ulonglong *)(puVar16 + 2);
    }
    else {
      puVar15 = *(ulonglong **)(puVar16 + 2);
    }
    if (-1 < (longlong)uVar12) break;
    FUN_140075720();
LAB_145420147:
    FUN_14e9380b0(&DAT_1528d6fc8);
    puVar17 = puVar21;
    if (_DAT_1528d6fc8 == -1) {
      FUN_14e934b40(FUN_140a771c0);
      FUN_14e938170(&DAT_1528d6fc8);
    }
  }
  if (uVar12 < 0x10) {
    uStack_a0 = 0xf;
    uStack_b8 = *puVar15;
    uStack_b0 = puVar15[1];
    puVar26 = puVar16;
    uStack_a8 = uVar12;
  }
  else {
    uVar20 = uVar12 | 0xf;
    uVar18 = 0x16;
    if (0x16 < uVar20) {
      uVar18 = uVar20;
    }
    puStack_58 = puVar11;
    if (uVar20 < 0xfff) {
      unaff_RBX = (ulonglong *)(uVar18 + 1);
      uVar20 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,unaff_RBX);
      if (uVar20 == 0) goto LAB_1454201f4;
    }
    else {
      unaff_RBX = (ulonglong *)(uVar18 + 0x28);
      lVar13 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,unaff_RBX);
      if (lVar13 == 0) {
        FUN_1409bc820(unaff_RBX);
        unaff_RBX = (ulonglong *)0x0;
      }
      uVar20 = lVar13 + 0x27U & 0xffffffffffffffe0;
      *(longlong *)(uVar20 - 8) = lVar13;
    }
    uStack_b8 = uVar20;
    uStack_a8 = uVar12;
    uStack_a0 = uVar18;
    FUN_14eec62c0(uVar20,puVar15,uVar12 + 1);
    puVar11 = puStack_58;
    puVar26 = puStack_60;
  }
  puVar15 = puStack_80;
  uStack_98 = puVar26[10];
  uStack_94 = puVar26[0xb];
  uStack_90 = puVar26[0xc];
  uStack_8c = puVar26[0xd];
  uStack_88 = 1;
  if ((uint)puStack_68 < 2) {
    puVar14 = puVar21 + 0xf;
  }
  else {
    puVar14 = puVar17;
    if (param_6 - 2 < 2) {
      puVar14 = puVar21 + 0xd;
    }
  }
  puVar24 = puStack_80 + -3;
  puVar16 = puVar26;
  if (*(longlong *)*puVar14 == 0) {
LAB_14541fdd5:
    if (*(char *)(puVar15[0x37] + 0x230) != '\0') goto LAB_14541fde5;
    if ((cStack_49 == '\0') && (puVar11 != (ulonglong *)0x0)) {
      FUN_145a4c3c0(auStack_168);
      puStack_e0 = &UNK_14f8469d4;
      uStack_d8 = 0;
      FUN_144df8eb0(&puStack_138,auStack_168,1);
      FUN_145420ec0(puVar24,puVar26,&puStack_138,&puStack_e0);
      lVar13 = lStack_130;
      if (lStack_130 != 0) {
        LOCK();
        piVar1 = (int *)(lStack_130 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_130);
          LOCK();
          piVar1 = (int *)(lVar13 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar13);
          }
        }
      }
      if (lStack_160 != 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
      }
    }
    else {
      uVar12 = FUN_1454776c0(puVar15 + 0x46,auStack_c0);
      if (uVar12 != puVar15[0x46]) {
        if (*(longlong *)(uVar12 + 0x68) == 0) {
          lVar13 = 0;
        }
        else {
          LOCK();
          piVar1 = (int *)(*(longlong *)(uVar12 + 0x68) + 8);
          *piVar1 = *piVar1 + 1;
          UNLOCK();
          lVar13 = *(longlong *)(uVar12 + 0x68);
        }
        unaff_RBX = *(ulonglong **)(uVar12 + 0x60);
        puStack_138 = unaff_RBX;
        lStack_130 = lVar13;
        if (unaff_RBX != (ulonglong *)0x0) {
          iVar10 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(unaff_RBX + 3);
          puVar26 = (undefined4 *)(ulonglong)(iVar10 != 2);
        }
        if (lVar13 != 0) {
          LOCK();
          piVar1 = (int *)(lVar13 + 8);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar13);
            LOCK();
            piVar1 = (int *)(lVar13 + 0xc);
            *piVar1 = *piVar1 + -1;
            UNLOCK();
            if (*piVar1 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar13);
            }
          }
        }
        puVar16 = puStack_60;
        if ((unaff_RBX != (ulonglong *)0x0 & (byte)puVar26) != 0) goto LAB_14541ff48;
      }
      FUN_145420400(puVar24,auStack_c0);
    }
  }
  else {
    if ((uint)puStack_68 < 2) {
      puVar14 = puVar21 + 0xf;
    }
    else {
      puVar14 = puVar17;
      if (param_6 - 2 < 2) {
        puVar14 = puVar21 + 0xd;
      }
    }
    unaff_RBX = *(ulonglong **)*puVar14;
    if (unaff_RBX == (ulonglong *)0x0) goto LAB_14541fdd5;
    if ((*(int *)(*(longlong *)(*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)__tls_index * 8)
                 + 4) < _DAT_1528d6fc8) &&
       (puStack_58 = puVar11, FUN_14e9380b0(&DAT_1528d6fc8), puVar11 = puStack_58,
       _DAT_1528d6fc8 == -1)) {
      FUN_14e934b40(FUN_140a771c0);
      FUN_14e938170(&DAT_1528d6fc8);
      puVar11 = puStack_58;
    }
    LOCK();
    *(short *)(unaff_RBX + 2) = (short)unaff_RBX[2] + 1;
    UNLOCK();
    if ((*(byte *)((longlong)unaff_RBX + 0x12) & 1) == 0) goto LAB_14541fdd5;
LAB_14541fde5:
    uVar12 = FUN_1454776c0(puVar15 + 0x46,auStack_c0);
    if (uVar12 != puVar15[0x46]) {
      if (*(longlong *)(uVar12 + 0x68) == 0) {
        lVar13 = 0;
      }
      else {
        LOCK();
        piVar1 = (int *)(*(longlong *)(uVar12 + 0x68) + 8);
        *piVar1 = *piVar1 + 1;
        UNLOCK();
        lVar13 = *(longlong *)(uVar12 + 0x68);
      }
      unaff_RBX = *(ulonglong **)(uVar12 + 0x60);
      puStack_138 = unaff_RBX;
      lStack_130 = lVar13;
      if (unaff_RBX != (ulonglong *)0x0) {
        iVar10 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(unaff_RBX + 3);
        puVar26 = (undefined4 *)(ulonglong)(iVar10 != 2);
      }
      if (lVar13 != 0) {
        LOCK();
        piVar1 = (int *)(lVar13 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar13);
          LOCK();
          piVar1 = (int *)(lVar13 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar13);
          }
        }
      }
      puVar16 = puStack_60;
      if ((unaff_RBX != (ulonglong *)0x0 & (byte)puVar26) != 0) goto LAB_14541ff48;
    }
    FUN_145420400(puVar24,auStack_c0);
  }
LAB_14541ff48:
  if (0xf < uStack_a0) {
    uVar12 = uStack_a0 + 1;
    uVar18 = uStack_b8;
    if (0xfff < uVar12) {
      uVar18 = *(ulonglong *)(uStack_b8 - 8);
      if (0x1f < (uStack_b8 - 8) - uVar18) goto LAB_1454201db;
      uVar12 = uStack_a0 + 0x28;
    }
    FUN_14e934b00(uVar18,uVar12);
  }
LAB_14541ff87:
  if (param_6 - 2 < 2) {
    puVar17 = puVar21 + 0xd;
    *puStack_70 = 0;
    puStack_70[1] = 0;
    if (puVar21[0xe] != 0) {
      LOCK();
      piVar1 = (int *)(puVar21[0xe] + 8);
      *piVar1 = *piVar1 + 1;
      UNLOCK();
      uVar12 = puVar21[0xe];
      goto LAB_14542000b;
    }
  }
  else if ((uint)puStack_68 < 2) {
    puVar17 = puVar21 + 0xf;
    *puStack_70 = 0;
    puStack_70[1] = 0;
    if (puVar21[0x10] != 0) {
      LOCK();
      piVar1 = (int *)(puVar21[0x10] + 8);
      *piVar1 = *piVar1 + 1;
      UNLOCK();
      uVar12 = puVar21[0x10];
      goto LAB_14542000b;
    }
  }
  else {
    *puStack_70 = 0;
    puStack_70[1] = 0;
    if (puVar21[0xc] != 0) {
      LOCK();
      piVar1 = (int *)(puVar21[0xc] + 8);
      *piVar1 = *piVar1 + 1;
      UNLOCK();
      uVar12 = puVar21[0xc];
      goto LAB_14542000b;
    }
  }
  uVar12 = 0;
LAB_14542000b:
  *puStack_70 = *puVar17;
  puStack_70[1] = uVar12;
  FUN_144df9790(puStack_70 + 2,puVar16);
  return puStack_70;
}

