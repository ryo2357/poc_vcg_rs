
#include <stdio.h>

int factorial(int n)
{
  // Cのコードは差分検知してビルドされない
  // cargo cleanでビルドを消すか
  // target/debug/build/クレートを消す
  // 全部消すとコンパイルにお時間がかかるからクレート分割はされてた方がいい
  printf("factorial in c\n");

  fprintf(stdout, "%d\n", n);

  if (n == 0)
    return 1;
  return n * factorial(n > 0 ? n - 1 : -n - 1);
}
