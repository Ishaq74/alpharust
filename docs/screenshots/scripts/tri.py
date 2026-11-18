import sys
from PIL import Image, ImageDraw
def tri(names, labels, out, scale=0.6):
    ims=[Image.open(n).convert('RGB') for n in names]
    ims=[i.resize((int(i.width*scale),int(i.height*scale))) for i in ims]
    h=max(i.height for i in ims); W=sum(i.width for i in ims)+20*(len(ims)-1)
    im=Image.new('RGB',(W,h+40),'white'); d=ImageDraw.Draw(im); x=0
    for i,l in zip(ims,labels):
        im.paste(i,(x,40)); d.text((x+10,12),l,fill='black'); x+=i.width+20
        if x<W: d.line([(x-10,0),(x-10,h+40)],fill='#999',width=2)
    im.save(out)
L=['Astro CSS pur (référence)','Laravel Blade','Django / Wagtail']
for k in sys.argv[1:]:
    tri([f'css-{k}.png',f'laravel-{k}.png',f'wagtail-{k}.png'],L,f'tri-{k}.png', 0.6 if 'advanced' in k else 1)
