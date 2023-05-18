#include "./meshlab.h"

#include <vcg/complex/algorithms/create/platonic.h>
#include <vcg/complex/complex.h>
#include <common/ml_document/cmesh.h>

using namespace vcg;
using namespace std;

int print_tetrahedron()
{
  CMeshO mesh;
  tri::Tetrahedron(mesh); // 正四面体を作成

  for (CMeshO::VertexType &vt : mesh.vert)
  { // 各頂点にアクセス
    cout << vt.Index() << endl;
  }

  return 0;
}