import cadquery as cq

class Cap:

    def __init__(self):
        pass

    def _srect(self, width, depth, delta=9, op="chamfer"):
        rect= (cq.Sketch().rect(width,depth))

        if op == "chamfer":
            return (rect.vertices().chamfer(delta))
        elif op == "fillet":
            return (rect.vertices().fillet(delta))
        else:
            return rect

    def _box(self, width, depth, height, diff=0, deltaA=9, deltaB=4, op="chamfer"):
        a = self._srect(width, depth, deltaA, op)
        b = self._srect(width+diff, depth+diff, deltaB, op)
        return (cq.Workplane("XY")
                .placeSketch(a, b.moved(cq.Location(cq.Vector(0,0,height))))
                .loft()
                )

    def _taperedCylinder(self, widthA, widthB, height):
        a = cq.Sketch().circle(widthA/2)
        b = cq.Sketch().circle(widthB/2)

        return (cq.Workplane("XY")
                .placeSketch(a, b.moved(cq.Location(cq.Vector(0,0,height))))
                .loft()
                )

    def stem(self, h, type="cherry"):
        return (cq.Workplane("XY")
                .sketch()
                .circle(5.6/2)
                .rect(1.2,4.2, mode="s")
                .rect(4.2,1.2, mode="s")
                .finalize()
                .extrude(h)
                .faces("<Z")
                .chamfer(0.25)
                )

    def row3(self):
        h = 8
        ih = h-2.4
        wd = 19
        diff = -7
        b = self._box(wd,wd, h, diff, 0,0, "none").fillet(0.7)
        hollow = self._box(wd-3,wd-3, ih, diff, 0 ,0, "none")
        dish = self._taperedCylinder(24,7,-2).translate((0,0,h+1))
        #.translate(cq.Location(cq.Vector(0,0,10)))

        b = b.cut(dish).cut(hollow)

        b = b.union(self.stem(ih,"cherry"))

        #b = b.faces(">>Y[13]").rect(2,4).extrude(5)
        #b = b.union(cq.Workplane("XY").move(0,-5.6/1.5).rect(2,3).extrude(3))
#        b = (b.faces(">Y[0]")
#                .sketch()
#                .rect(1,7)
#                .finalize()
#                .extrude(-3)
#                )

        b = b.translate((0,0,-h/2))
        return b

d=7.97
dsa1u = (cq.importers.importStep('DSA_1u.step')
 .rotate((0,0,0),(1,0,0),90)
 .translate((0,0,-d/2))
 )

cap = Cap()

c = cap.row3()
show_object(c, options={"alpha":0, "color":(255,10,50)})
#show_object(dsa1u)
cq.exporters.export(c, "cap_row3.step", cq.exporters.ExportTypes.STEP)
cq.exporters.export(c, "cap_row3.stl")
