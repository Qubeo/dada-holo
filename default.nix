let 
  holonixPath = builtins.fetchTarball {
    url = "https://github.com/holochain/holonix/archive/f8bf0836b83ec6515e9da66a29302f18d298bbfa.tar.gz";
    sha256 = "1y3ligglx22yigjlhv0sman24arz2dqilrprp1nlnz5r02xsl13d";
  };
  holonix = import (holonixPath) {
    includeHolochainBinaries = true;
    holochainVersionId = "custom";
    
    holochainVersion = { 
     rev = "945d4e8c62c03b826794f41eea76efccdb3f0d1e";
     sha256 = "16a9mjq8g40mp5120714ff5wpks74n3n39liswikqqf1sxabza9c";
     cargoSha256 = "1fml9bngmp9dsgcrly0vavvdgii6fkcj4mjyakbck45kd8xx449d";
     bins = {
       holochain = "holochain";
       hc = "hc";
     };
    };
  };
in holonix.main
