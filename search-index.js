var searchIndex = JSON.parse('{\
"sphrs":{"doc":"A general purpose spherical/solid harmonics library in …","t":[4,3,3,13,13,4,13,13,5,8,8,13,13,8,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,5,11,10,11,10,11,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,11,10,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,10,11,10,11],"n":["ComplexSHType","Coordinates","HarmonicsSet","IrregularSolid","IrregularSolid","RealSHType","RegularSolid","RegularSolid","SH","SHCoordinates","SHEval","Spherical","Spherical","SphrsFloat","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","cartesian","clone","clone","clone","clone_into","clone_into","clone_into","default","eval","eval","eval","eval","fmt","from","from","from","from","into","into","into","into","irregular_solid_SH","new","phi","phi","r","r","real_SH","real_SH_hardcoded","real_irregular_solid_SH","real_regular_solid_SH","regular_solid_SH","sh00","sh10","sh1n1","sh1p1","sh20","sh2n1","sh2n2","sh2p1","sh2p2","sh30","sh3n1","sh3n2","sh3n3","sh3p1","sh3p2","sh3p3","spherical","theta","theta","theta_cos","theta_cos","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","with_coefficients","x","x","y","y","z","z"],"q":["sphrs","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Available types of complex spherical harmonics and solid …","Representation of coordinates.","A set of spherical/solid harmonics up to a given degree","Irregular solid harmonics","Irregular solid harmonics","Available types of real spherical harmonics and solid …","Regular solid harmonics","Regular solid harmonics","Complex spherical harmonics","SHCoordinates trait","SH eval trait (TODO)","Spherical harmonics","Spherical harmonics","Trait alias to simplify common trait bounds","","","","","","","","","Create <code>Coordinates</code> struct from Cartesian coordinates","","","","","","","","Evaluate SH (l, m) at position <code>p</code>","Evaluate harmonics at position <code>p</code>. This will respect …","Evaluate real SH (l, m) at position <code>p</code>","Evaluate complex SH (l, m) at position <code>p</code>","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Complex irregular solid harmonics","Create new <code>HarmonicsSet</code> struct","Return <code>phi</code> (spherical coordinates)","Return angle <code>phi</code>","Return radus <code>r</code> (spherical coordinates)","Return radius <code>r</code>","Real spherical harmonics (recursive implementation)","Spherical harmonics. This will use the hardcoded functions …","Real irregular solid harmonics","Real regular solid harmonics","Complex regular solid harmonics","Hardcoded SH (l=0,m=0)","Hardcoded SH (l=1,m=0)","Hardcoded SH (l=1,m=-1)","Hardcoded SH (l=1,m=1)","Hardcoded SH (l=2,m=0)","Hardcoded SH (l=2,m=-1)","Hardcoded SH (l=2,m=-2)","Hardcoded SH (l=2,m=1)","Hardcoded SH (l=2,m=2)","Hardcoded SH (l=3,m=0)","Hardcoded SH (l=3,m=-1)","Hardcoded SH (l=3,m=-2)","Hardcoded SH (l=3,m=-3)","Hardcoded SH (l=3,m=1)","Hardcoded SH (l=3,m=2)","Hardcoded SH (l=3,m=3)","Create <code>Coordinates</code> struct from spherical coordinates","Return <code>theta</code> (spherical coordinates)","Return angle <code>theta</code>","Return <code>cos(theta)</code>","Return <code>cos(theta)</code>","","","","","","","","","","","","","","","","Add coefficients","Return <code>x</code> (Cartesian coordinates)","Return coordinate <code>x</code>","Return <code>y</code> (Cartesian coordinates)","Return coordinate <code>y</code>","Return <code>z</code> (Cartesian coordinates)","Return coordinate <code>z</code>"],"i":[0,0,0,7,8,0,7,8,0,0,0,7,8,0,10,5,7,8,10,5,7,8,5,5,7,8,5,7,8,5,18,10,7,8,5,10,5,7,8,10,5,7,8,0,10,2,5,2,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,2,5,2,5,5,7,8,10,5,7,8,10,5,7,8,10,5,7,8,10,2,5,2,5,2,5],"f":[0,0,0,0,0,0,0,0,[[1,1,2],[[4,[3]]]],0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],5],[[[5,[6]]],[[5,[6]]]],[7,7],[8,8],[[]],[[]],[[]],[[],[[5,[9]]]],[[1,1,2]],[10,11],[[7,1,1,2]],[[8,1,1,2],4],[[[5,[12]],13],14],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[1,1,2],[[4,[3]]]],[15,10],[[]],[5],[[]],[5],[[1,1,2],3],[[1,1,2],3],[[1,1,2],3],[[1,1,2],3],[[1,1,2],[[4,[3]]]],[2,3],[2,3],[2,3],[2,3],[2,3],[2,3],[2,3],[2,3],[2,3],[2,3],[2,3],[2,3],[2,3],[2,3],[2,3],[2,3],[[],5],[[]],[5],[[]],[5],[[]],[[]],[[]],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],16],[[],17],[[],17],[[],17],[[],17],[[10,11],10],[[]],[5],[[]],[5],[[]],[5]],"p":[[15,"i64"],[8,"SHCoordinates"],[8,"SphrsFloat"],[3,"Complex"],[3,"Coordinates"],[8,"Clone"],[4,"RealSHType"],[4,"ComplexSHType"],[8,"Default"],[3,"HarmonicsSet"],[3,"Vec"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[15,"usize"],[4,"Result"],[3,"TypeId"],[8,"SHEval"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
