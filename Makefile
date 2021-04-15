.PHONY: all
all: proposal.pdf assignment-2.pdf assignment-3.pdf assignment-4.pdf

proposal.pdf: proposal.tex egbib.bib
	true | pdflatex -halt-on-error -shell-escape 'proposal'
	true | bibtex 'proposal'
	true | pdflatex -halt-on-error -shell-escape 'proposal'
	true | pdflatex -halt-on-error -shell-escape 'proposal'

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
	true | pdflatex -halt-on-error -shell-escape 'assignment-4'
	true | bibtex 'assignment-4'
	true | pdflatex -halt-on-error -shell-escape 'assignment-4'
	true | pdflatex -halt-on-error -shell-escape 'assignment-4'
