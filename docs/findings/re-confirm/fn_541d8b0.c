
bool FUN_14541d8b0(longlong param_1,undefined8 param_2)

{
  longlong lVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong lVar5;
  
  lVar3 = FUN_140ab6b70((longlong *)(param_1 + 0x1a8));
  if (*(longlong *)(param_1 + 0x240) != 0) {
    FUN_141127150(*(longlong *)(param_1 + 0x240),param_2);
  }
  lVar2 = *(longlong *)(param_1 + 0x1a8);
  if (lVar3 != lVar2) {
    FUN_145676fe0(lVar3 + 0x58);
    lVar1 = param_1 + 0x230;
    lVar5 = FUN_140ab6b70(lVar1,lVar3 + 0x20);
    if (lVar5 != *(longlong *)(param_1 + 0x230)) {
      uVar4 = FUN_140132910(lVar1,lVar5);
      FUN_1411670c0(lVar1,uVar4);
    }
  }
  return lVar3 != lVar2;
}

