import java.util.*;

class SlideShow {
	public static void main(String[] args) {
		Set<String> ctags = new HashSet<String>();
		Scanner in = new Scanner(System.in);
		int t = Integer.parseInt(in.nextLine());
		Photo[] Photos = new Photo[t];
		for(int i=0; i<t; i++) {
			String[] photodef = in.nextLine().split(" ");
			Photos[i] = new Photo(i, photodef[0].charAt(0),
														Integer.parseInt(photodef[1]),
														Arrays.copyOfRange(photodef, 2, photodef.length));
			System.out.println(Photos[i].index);
			System.out.println(Photos[i].orient);
			System.out.println(Photos[i].tagcount);
			for (String ctag : Arrays.copyOfRange(photodef, 2, photodef.length)) {
				ctags.add(ctag);
			}
			for (String tag : Photos[i].tags) {
				System.out.println(tag);
			}
		}
		for (String ctag : ctags) {
			System.out.println(ctag);
		}
	}
}

class Photo {
	int index;
	char orient;
	int tagcount;
	Set<String> tags;

	Photo(int index, char orient, int tagcount, String[] tags) {
		this.index = index;
		this.orient = orient;
		this.tagcount = tagcount;
		this.tags = new HashSet<String>();
		for (String tag : tags) {
			this.tags.add(tag);
		}
	}
}

