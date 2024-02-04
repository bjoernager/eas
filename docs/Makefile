TEXARGS := --output-directory=build

BUILDDIR := build

DOC := eas

LOGO := eas

$(BUILDDIR)/$(DOC).pdf: $(DOC).tex $(BUILDDIR)/$(DOC).png $(BUILDDIR)
	pdflatex $(TEXARGS) $(DOC).tex
	pdflatex $(TEXARGS) $(DOC).tex

$(BUILDDIR)/eas.png: $(LOGO).svg $(BUILDDIR)
	inkscape --export-type=png --export-width=1024 --export-filename=$(BUILDDIR)/$(LOGO).png $(LOGO).svg

$(BUILDDIR):
	mkdir -p $(BUILDDIR)
