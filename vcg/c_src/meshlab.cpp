#include "meshlab.h"

int print_tetrahedron()
{
  CMeshO mesh;
  tri::Tetrahedron(mesh); // 正四面体を作成

  for (CMeshO::VertexType &vt : mesh.vert)
  { // 各頂点にアクセス
    cout << vt.Index() << endl;
  }

  return 1;
}