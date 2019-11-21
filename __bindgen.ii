template < class > struct a;
template < class b > struct a< b * > {
  typedef b c;
  template < class i > using d = i *;
};
template < class e, class f > struct g {
  typedef typename a< e >::template d< f > h;
};
template < class k > struct l { typedef k h; };
template < class > struct m { typedef l< int * >::h n; };
template < class, class > class __tree_node;
template < class o, class = typename a< o >::c > struct p;
template < class o, class b, class q > struct p< o, __tree_node< b, q > > {
  typedef o __node_pointer;
};
template < class q > struct r {
  typedef p< typename g< q, __tree_node< int, q > >::h > h;
};
template < class > class s;
template < class t > class w {
  typedef typename r< typename m< t >::n >::h x;

public:
  typedef typename x::__node_pointer __node_pointer;
  typedef s< __node_pointer > u;
};
template < class > class y {};
namespace octave {
class stream {
  y< w< int >::u > v;
};
} 