
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_14541b8a0(longlong param_1,undefined8 param_2,char param_3,uint param_4)

{
  longlong lVar1;
  undefined8 *puVar2;
  longlong lVar3;
  ulonglong uVar4;
  longlong unaff_GS_OFFSET;
  bool bVar5;
  
  lVar3 = param_1 + 0x1a8;
  lVar1 = FUN_140ab6b70(lVar3);
  bVar5 = lVar1 != *(longlong *)(param_1 + 0x1a8);
  uVar4 = CONCAT71((int7)((ulonglong)lVar3 >> 8),bVar5);
  if (bVar5 && param_3 == '\0') {
    if ((param_4 & 0xff) - 8 < 2) {
      puVar2 = (undefined8 *)(lVar1 + 0x78);
    }
    else if ((param_4 & 0xff) - 2 < 2) {
      puVar2 = (undefined8 *)(lVar1 + 0x68);
    }
    else {
      puVar2 = (undefined8 *)(lVar1 + 0x58);
    }
    lVar1 = *(longlong *)*puVar2;
    if (lVar1 == 0) {
      uVar4 = 0;
    }
    else {
      if (*(int *)(*(longlong *)(*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)__tls_index * 8)
                  + 4) < _DAT_1528d6fc8) {
        FUN_14e9380b0(&DAT_1528d6fc8);
        if (_DAT_1528d6fc8 == -1) {
          FUN_14e934b40(FUN_140a771c0);
          FUN_14e938170(&DAT_1528d6fc8);
        }
      }
      LOCK();
      *(short *)(lVar1 + 0x10) = *(short *)(lVar1 + 0x10) + 1;
      UNLOCK();
      uVar4 = (ulonglong)*(byte *)(lVar1 + 0x12);
    }
  }
  return uVar4 & 0xffffff01;
}

