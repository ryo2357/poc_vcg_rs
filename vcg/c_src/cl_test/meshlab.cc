#include "./meshlab.h"

#include <vcg/complex/algorithms/create/platonic.h>
#include <vcg/complex/complex.h>
// ここが問題でCMeshOが参照できない
#include <common/ml_document/cmesh.h>
// #include <common/ml_mesh_type.h>
// #include <common/ml_document/base_types.h>

using namespace vcg;
using namespace std;

int main()
{
  CMeshO mesh;
  tri::Tetrahedron(mesh); // 正四面体を作成

  for (CMeshO::VertexType &vt : mesh.vert)
  { // 各頂点にアクセス
    cout << vt.Index() << endl;
  }

  return 0;
}