import math


def eur_to_usd():
    rate = 1.11
    eur = int(input("Vnesi evre: "))
    with open("out.txt", "a") as f:
        print("{} EUR je enako {:.2f} USD".format(eur, round(eur*rate)), file=f)
        print("", file=f)

def nakup():
    kolicinaDenarja = int(input("Kolicina denarja: "))
    cenaIzdelka = int(input("Cena izdelka: "))
    with open("out.txt", "a") as f:
        print("Imam {} evrov, zato lahko kupim {} izdelke, po ceni {} evrov.".format(kolicinaDenarja, kolicinaDenarja//cenaIzdelka,cenaIzdelka),file=f)
        print("", file=f)

def vsota():
    arr = [4, 7, 1, 3]
    arr_str = ", ".join(str(x) for x in arr)
    with open("out.txt", "a") as f:
        print( "Vsota števil {} je {}".format(arr_str, sum(arr)), file=f)
        print("", file=f)

def ure_minute_sekunde():
    sekunde = int(input("Vnesi sekunde: "))
    ure = sekunde // 60**2
    minute = sekunde // 60 - ure * 60
    sekunde2 = sekunde - ure * 60**2 - minute * 60
    with open("out.txt", "a") as f:
        print("%d sekund je enako %d ur, %d minut in %d sekund" % (sekunde, ure, minute, sekunde2), file=f)
        print("", file=f)
def krog():
    r = float(input("Vnesi radij: "))
    pi = 3.14
    with open("out.txt", "a") as f:
        print(f"Obseg kroga s polmerom {r} cm je {2*pi*r:.2f} cm, ploscina pa {pi*r**2:.2f} cm2.", file=f)
        print("", file=f)
def deljivo():
    stevilke = [10 , 15, 23, 42, 55]
    delitelj = 5
    deljiva_st = [x for x in stevilke if x % delitelj == 0]
    with open("out.txt", "a") as f:
        print("stevilke = %s\ndelitelj: %d\ndeljiva_st = %s" % (str(stevilke), delitelj, str(deljiva_st)),file=f)
        print("", file=f)
def cas():
    from datetime import datetime as dt
    formated = dt.now().strftime("%d.%m.%Y %H:%M")
    with open("out.txt", "a") as f:
        print( f"Trenutni datum in ura: {formated}", file=f)
        print("", file=f)
def hipotenuza():
    k1 = int(input("K1: "))
    k2 = int(input("K2: "))
    with open("out.txt", "a") as f:
        print("V pravokotnem trikotniku s katetama {} in {} cm meri hipotenuza {:.2f} cm".format(k1, k2, math.sqrt(k1**2 + k2 ** 2)), file=f)
        print("", file=f)

def komplekni():
    c1 = complex(input("Vnesi c1: "))
    c2 = complex(input("Vnesi c2: "))
    vsota = c1 +c2
    razlika = c1 -c2
    k = c1 *c2
    with open("out.txt", "a") as f:
        print("Vsota: %dj+%d" % (vsota.real,vsota.imag), file=f)
        print("Razlika: %dj+%d" % (razlika.real,razlika.imag), file=f)
        print("Produkt: %dj+%d" % (k.real, k.imag), file=f)
        print("", file=f)

def kvadratna(e):
    import math
    s = e.split(" ")
    a = int(s[0].replace("x2", ""))
    b = int(s[2].replace("x", ""))
    c = int(s[4])

    d = b**2 - 4*a*c
    with open("out.txt", "a") as f:
        if d > 0:
            print("x1=%d, x2=%d" % ((-b-math.sqrt(d)) / 2*a,(-b+math.sqrt(d))/(2*a) ), file=f)
        elif d==0:
            print("x=%d" % ((-b - math.sqrt(d)) / (2 * a)),file=f)
        else:
            print("Nima realne resitve",file=f)
        print(file=f)



def geo():
    k = int(input("Začetni :"))
    a = int(input("Količnik :"))

    s = 0
    for i in range(1, 6):
        s += a * k**(i-1)
    with open("out.txt", "a") as f:
        print(f"Vsota prvih 5 clenov geometrijskega zaporedja z zacetnim clenom {a} in kolicnikom {k} je {s}.", file=f)
        print("", file=f)
def matrika():
    a = [[1, 2],[3, 4]]
    b = [[5,6],[7, 8]]

    s = [[a[0][0] + b[0][0], a[0][1] + b[0][1]], [a[1][0] + b[1][0], a[1][1] + b[1][1]]]
    with open("out.txt", "a") as f:
        print("Vsota matrix A in B:", file=f)
        print(f"[{s[0][0]}, {s[0][1]}]", file=f)
        print(f"[{s[1][0]}, {s[1][1]}]", file=f)
        print("", file=f)
eur_to_usd()
nakup()
vsota()
ure_minute_sekunde()
krog()
deljivo()
cas()
hipotenuza()
kvadratna("2x2 + 4x + 2 = 0")
komplekni()
geo()
matrika()

