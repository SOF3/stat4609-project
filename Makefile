.PHONY: all
all: proposal.pdf

proposal.pdf: proposal.tex
	true | pdflatex -halt-on-error -shell-escape 'proposal'
	true | bibtex 'proposal'
	true | pdflatex -halt-on-error -shell-escape 'proposal'
	true | pdflatex -halt-on-error -shell-escape 'proposal'
