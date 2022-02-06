import cadquery as cq

class waddle_alt: 
    space = 19.05
    cherryCutOutSize = 14.05
    cherrySize = 14.58
    cols = 12
    rows = 4
    
    lip = 25
    fillet = 3
    width = cols * space + lip
    depth = rows * space + lip
    
    bottomHeight = 9
    topHeight = 16
    
    plateW = width - lip/1.5;
    plateD = depth - lip/1.2;
    plateThickness = 3

    def __init__(self):
        pass

    def cherryCut(self):
        return (cq.Workplane("XY")
                 .box(self.cherryCutOutSize, self.cherryCutOutSize, self.plateThickness+2)
    
                 # Side cuts
                 .faces(">Z")
                 .edges("|Y")
                 .rect(1,4)
                 .cutThruAll()
    
                 # Potrusions
                 .faces(">Y or <Y")
                 .edges("<Z")
                 .rect(4,1)
                 .extrude(self.plateThickness+1)
    
                 # Add chamfer to potrusions
                 .faces(">Y or <Y")
                 .edges(">Z")
                 .chamfer(0.2,0.4)
                 )
    def simpleCut(self):
        return (cq.Workplane("XY")
                 .box(self.cherryCutOutSize, self.cherryCutOutSize, self.plateThickness+2)
                 )
    
    def plate(self, cut=cherryCut):
        return (cq.Workplane("XY")
                .rect(self.plateW, self.plateD)
                .extrude(self.plateThickness)
                .edges("|Z")
                .fillet(self.fillet)
                .workplane()
                .rarray(self.space, self.space, self.cols, self.rows, True)
                .cutEach(lambda loc: cut().val().moved(loc).translate((0,0,-0.25)))
                )
    
    def switches(self):
        # Cherry MX reference model by https://github.com/ConstantinoSchillebeeckx/cherry-mx-switch
        cherry = (cq.importers.importStep('cherry_mx.stp')
              .rotate((0,0,0),(1,0,0),90)
              .translate((0,-1,-1))
              )
    # cherry = cq.Workplane("XY").box(15.6,15.6,3.6+11.6+3.3)
        return (cq.Workplane()
                .rect(self.width+(self.lip/2),self.depth+(self.lip/2), forConstruction=True)
                .rarray(self.space,self.space,self.cols,self.rows,True)
                .eachpoint(lambda loc: cherry.val().moved(loc))
                .combine(glue=True)
              )
    
    def caps(self):
        d=7.97
        # DSA keycap model by Kael Ruland https://www.reddit.com/r/MechanicalKeyboards/comments/5yzsaz/dsa_keycap_i_modeled_print_files_available_in/
        dsa1u = (cq.importers.importStep('DSA_1u.step')
             .rotate((0,0,0),(1,0,0),90)
             .translate((0,0,-d/2))
             )
        # dsa1u = cq.Workplane("XY").box(15.6,15.6,10)
        return (cq.Workplane()
                .rect(self.width,self.depth, forConstruction=True)
                .rarray(self.space,self.space,self.cols,self.rows,True)
                .eachpoint(lambda loc: dsa1u.val().moved(loc))
                .combine(glue=True)
                # .translate((-4,1.6,0))
              )
    
    def proMicro(self, variant="c"):
        return (cq.importers.importStep("proMicro_"+variant+".step")
                #.rotate((0,0,0),(0,1,0),180)
                .rotate((0,0,0),(0,0,1),90)
                .translate((0,0,-1.5))
                )
    
    def socket(self):
        return (cq.importers.importStep("DIL_socket_24pins.step")
                .rotate((0,0,0),(0,1,0),180)
                .rotate((0,0,0),(0,0,1),90)
                )

    def bottom(self):
        return (cq.Workplane("XY")
                .sketch()
                .rect(self.width,self.depth)
                .vertices()
                .chamfer(9)
                .finalize()
                .extrude(-(self.bottomHeight), taper=40)
                .edges("<Z")
                .fillet(1)


                # Inner raised edge to rest the plate on
                .faces(">Z")
                .workplane()
                .tag("top")
                .sketch()
                .rect(self.plateW+2, self.plateD+2)
                .rect(self.plateW-5, self.plateD-5, mode="s")
                .vertices()
                .fillet(self.fillet)
                .finalize()
                .extrude(3)

                # Gut the bottom
                .workplaneFromTagged("top")
                .sketch()
                .rect(self.plateW-5, self.plateD-5)
                .vertices()
                .fillet(self.fillet)
                .finalize()
                .extrude(-(self.bottomHeight-5), "cut")

                # Place for proMicro + socket
                .faces("<Z")
                .edges("<X")
                .workplane(centerOption="CenterOfMass", offset=-1)
                .tag("pmcut")
                .center(35/2,self.space/2)
                .rect(36,19)
                .extrude(-10, "cut")
                
                # Punch a hole in the wall
                .first()
                .faces("<X")
                .workplane(offset=0)
                #.center(0,26)
                .sketch()
                .rect(9.8,4.1)
                .vertices()
                .fillet(1)
                .finalize()
                .extrude(5)#, "cut", taper=11)
                )

    def top(self):# {{{
        return (cq.Workplane("XY")
                .sketch()
                .rect(self.width,self.depth)
                .vertices()
                .chamfer(9)
                .finalize()
                .extrude(self.topHeight, taper=20)
                .edges(">Z")
                .fillet(1)

                # Cut a hole for switches
                .faces(">Z")
                .workplane()
                .rect(self.width-self.lip+1, self.depth-self.lip+1)
                .cutThruAll()

                # Shelf for plate
                .faces("<Z")
                .workplane()
                .tag("bottom")
                .sketch()
                .rect(self.plateW+2, self.plateD+2)
                .vertices()
                .fillet(self.fillet)
                .finalize()
                .extrude(-8, "cut")


                # Extra spacing so the bottom fits
                .workplaneFromTagged("bottom")
                .sketch()
                .rect(self.plateW+4, self.plateD+4)
                .vertices()
                .fillet(self.fillet)
                .finalize()
                .extrude(-3.4, "cut")
                )# }}}


def keyboard(bottom=False
        , plate=False
        , switches=False
        , caps=False
        , proMicro=False
        , socket=False
        , top=False
        ):
    
    _waddle = waddle_alt()
    _bottom = _waddle.bottom()
    bbb = _bottom.findSolid().BoundingBox()
    sock = _waddle.socket()
    sockBox = sock.findSolid().BoundingBox()

    pmx = bbb.xmin+sockBox.xmax+9.5
    pmy = -_waddle.space/2
    pmz = -4

    topZ = 0#_waddle.bottomHeight
    plateZ = topZ + 4.0
    switchZ = plateZ + _waddle.plateThickness + 1.4
    capsZ = switchZ + 9.5


    kb = cq.Assembly()

    if bottom:
        kb.add(_bottom
                , name  = "bottom"
                , color = cq.Color(0,0,1,0.5)
                , loc   = cq.Location(cq.Vector(0,0,0))
                )
    if plate:
        kb.add(_waddle.plate(_waddle.simpleCut)
                , name  = "plate"
                , color = cq.Color(1,1,0,1)
                , loc   = cq.Location(cq.Vector(0,0,plateZ))
                )
    if switches:
        kb.add(_waddle.switches()
                , name  = "switches"
                , color = cq.Color(0,1,0,1)
                , loc   = cq.Location(cq.Vector(0,0,switchZ))
                )
    if caps:
        kb.add(_waddle.caps()
                , name  = "caps"
                , color = cq.Color(1,0,1,1)
                , loc   = cq.Location(cq.Vector(0,0,capsZ))
                )
    if proMicro:
        kb.add(_waddle.proMicro("c")
                , name  = "proMicro"
                , color = cq.Color(1,1,0.3,1)
                , loc   = cq.Location(cq.Vector(pmx,pmy,pmz))
                )
    if socket:
        kb.add(sock
                , name  = "socket"
                , color = cq.Color(0.3,0.5,0.3,1)
                , loc   = cq.Location(cq.Vector(pmx,pmy,pmz))
                )
    if top:
        kb.add(_waddle.top()
                , name  = "top"
                , color = cq.Color(0,1,1,1)
                , loc   = cq.Location(cq.Vector(0,0,topZ))
                )
    return kb

k = keyboard(bottom=True
        , plate=False
#        , switches=True
#        , caps=True
        , proMicro=True
#        , socket=True
#        , top=True
        )
show_object(k)
k.save("waddle_alt.step")
