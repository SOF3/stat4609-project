.PHONY: all
all: assignment-1.pdf assignment-2.pdf assignment-3.pdf assignment-4.pdf assignment-5.pdf

assignment-1.pdf: assignment-1.tex egbib.bib
	true | pdflatex -halt-on-error -shell-escape 'assignment-1'
	true | bibtex 'assignment-1'
	true | pdflatex -halt-on-error -shell-escape 'assignment-1'
	true | pdflatex -halt-on-error -shell-escape 'assignment-1'

assignment-2.pdf: assignment-2.tex egbib.bib
	true | pdflatex -halt-on-error -shell-escape 'assignment-2'
	true | bibtex 'assignment-2'
	true | pdflatex -halt-on-error -shell-escape 'assignment-2'
	true | pdflatex -halt-on-error -shell-escape 'assignment-2'

assignment-3.pdf: assignment-3.tex egbib.bib
	true | pdflatex -halt-on-error -shell-escape 'assignment-3'
	true | bibtex 'assignment-3'
	true | pdflatex -halt-on-error -shell-escape 'assignment-3'
	true | pdflatex -halt-on-error -shell-escape 'assignment-3'

assignment-4.pdf: assignment-4.tex egbib.bib
	rm *.bbl *.blg *.brf || true
	true | pdflatex -halt-on-error -shell-escape 'assignment-4'
	true | bibtex 'assignment-4'
	true | pdflatex -halt-on-error -shell-escape 'assignment-4'
	true | pdflatex -halt-on-error -shell-escape 'assignment-4'

assignment-5.pdf: assignment-5.tex egbib.bib
	rm *.bbl *.blg *.brf || true
	true | pdflatex -halt-on-error -shell-escape 'assignment-5'
	true | bibtex 'assignment-5'
	true | pdflatex -halt-on-error -shell-escape 'assignment-5'
	true | pdflatex -halt-on-error -shell-escape 'assignment-5'
