# FCM to SVG converter

based on the wonderful library fcmlib https://github.com/justjanne/fcmlib
this project tries to convert the paths to a svg file.

The main purpose for me is to make the proprietary file format readable
and to archive files I created in an interchangable format.

This is my first try to code in Rust. Forgive me for my errors.


## Internal hints (will be removed once complete)

Maße in der Original-Datei

Eine Gruppe
 * Rechteck x=12mm, y=8,8mm, w=275mm, h=153,4mm

fcm Datei
 * CutData cut_width: 29667, cut_height: 29880,
 * Piece width: 27500, height: 15336
   transform 1,0, 0,1, 14645.671,8246.482
   Paths:
     * 3762,-922 -> ...
     * ...
     * Rect 13751,-7668 -> 13751,7669 -> -13750,7669 -> -13751,-7668, 13751,-7668
       => Breite 27500, Höhe 15336
       => Maßeinheit 1/100 mm

svg Datei (aus Affinity Designer für Shaper Origin)
 * svg: width=842px == 210mm, height=596px == 297mm
 * rect x=34 == 12mm, y=25 == 8,8mm
   width=779 == 274mm, height=434 == 153mm

Berechnungen:
 * Pixel -> mm
   25,4 / 72 * pt => mm
 * FCM -> Pixel
   fcm Maße / 100 [mm] / 25.4 * 72 [pt]
   = fcm Maße * 0.028346
   = fcm Maße / 35,277777

SVG Beispiel
      <?xml version="1.0" encoding="UTF-8" standalone="no"?>
      <!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
      <svg width="842px" height="596px" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5;">
          <rect x="34.016" y="24.945" width="779.528" height="434.835" style="fill:rgb(235,235,235);"/>
          <path d="M124.293,146.559C82.164,104.528 214.574,57.062 216.526,121.905C217.2,144.269 208.813,182.905 211.887,197.711C219.03,232.115 307.57,201.33 304.488,262.148C303.171,288.138 272.762,317.828 240.259,329.252C204.496,341.821 157.431,327.377 154.388,326.654" style="fill:rgb(235,235,235);stroke:black;stroke-width:0.24px;"/>
      </svg>
